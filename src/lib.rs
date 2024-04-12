use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
  spanned::Spanned, Error, Fields, ItemStruct, Result, Type, Visibility,
};

#[proc_macro_attribute]
pub fn typed(
  params: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  match typed_main(params.into(), input.into()) {
    Ok(output) => output.into(),
    Err(err) => err.into_compile_error().into(),
  }
}

fn typed_main(params: TokenStream, input: TokenStream) -> Result<TokenStream> {
  // TODO: allow unit struct + inner type as macro param.
  if !params.is_empty() {
    return Err(Error::new(
      Span::call_site(),
      String::from("This macro does not (yet) accept parameters"),
    ));
  }

  let st = StrongType::parse(input)?;

  let mut output = TokenStream::new();

  for codegen in GENERATORS {
    output.extend(codegen.dispatch(&st)?);
  }

  st.emit_impl(&mut output)?;
  st.emit_impl_ops_alg(&mut output)?;
  if st.inner_base.is_int() {
    st.emit_impl_ops_bit(&mut output)?;
    st.emit_impl_int_display(&mut output)?;
  }

  // TODO: serde macro param
  if cfg!(feature = "serde") {
    st.emit_impl_serde(&mut output)?;
  }

  Ok(output)
}

struct StrongType {
  input: TokenStream,
  outer: Ident,
  outer_vis: Visibility,
  inner: Ident,
  inner_base: BaseType,
  inner_vis: Visibility,
}

impl StrongType {
  fn parse(input: TokenStream) -> Result<Self> {
    let item = syn::parse2::<ItemStruct>(input.clone())?;

    let outer = item.ident.clone();
    // TODO: find out why dtolnay doesn't like Clone here.
    let outer_vis = syn::parse2(item.vis.to_token_stream())?;

    let Fields::Unnamed(fields) = &item.fields else {
      return Err(Error::new(item.span(), "not tuple struct"));
    };
    if fields.unnamed.len() != 1 {
      return Err(Error::new(item.span(), "not newtype struct"));
    }
    let field = fields.unnamed.first().expect("first element");
    let inner = match &field.ty {
      Type::Path(path) => path.path.require_ident()?.clone(),
      _ => return Err(Error::new(field.ty.span(), "unexpected type")),
    };
    let inner_base = match inner.to_string().as_str() {
      "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => {
        BaseType::Int { signed: false }
      }
      "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => {
        BaseType::Int { signed: true }
      }
      "f32" | "f64" => BaseType::Float,
      "char" => BaseType::Char,
      "bool" => BaseType::Bool,
      _ => return Err(Error::new(field.ty.span(), "unsupported inner type")),
    };
    let inner_vis = syn::parse2(field.vis.to_token_stream())?;

    Ok(StrongType { input, outer, outer_vis, inner, inner_base, inner_vis })
  }
}

#[derive(Copy, Clone, Debug)]
enum BaseType {
  Int { signed: bool },
  Float,
  Char,
  Bool,
}

impl BaseType {
  fn is_int(&self) -> bool {
    matches!(self, BaseType::Int { .. })
  }

  fn is_float(&self) -> bool {
    matches!(self, BaseType::Float)
  }

  fn is_char(&self) -> bool {
    matches!(self, BaseType::Char)
  }

  fn is_bool(&self) -> bool {
    matches!(self, BaseType::Bool)
  }

  fn is_signed(&self) -> bool {
    matches!(self, BaseType::Int { signed: true })
  }

  fn parse_err_tokens(&self) -> TokenStream {
    match self {
      Self::Int { .. } => quote!(::core::num::ParseIntError),
      Self::Float => quote!(::core::num::ParseFloatError),
      Self::Char => quote!(::core::char::ParseCharError),
      Self::Bool => quote!(::core::str::ParseBoolError),
    }
  }
}

trait CodeGenerator: Sync + Send {
  fn dispatch(&self, st: &StrongType) -> Result<TokenStream> {
    match st.inner_base {
      BaseType::Int { signed: false } => self.emit_unsigned_int(st),
      BaseType::Int { signed: true } => self.emit_signed_int(st),
      BaseType::Float => self.emit_float(st),
      BaseType::Char => self.emit_char(st),
      BaseType::Bool => self.emit_bool(st),
    }
  }

  fn emit(&self, st: &StrongType) -> Result<TokenStream>;

  fn emit_unsigned_int(&self, st: &StrongType) -> Result<TokenStream> {
    self.emit_int(st)
  }

  fn emit_signed_int(&self, st: &StrongType) -> Result<TokenStream> {
    self.emit_int(st)
  }

  fn emit_int(&self, st: &StrongType) -> Result<TokenStream> {
    self.emit(st)
  }

  fn emit_float(&self, st: &StrongType) -> Result<TokenStream> {
    self.emit(st)
  }

  fn emit_char(&self, st: &StrongType) -> Result<TokenStream> {
    self.emit(st)
  }

  fn emit_bool(&self, st: &StrongType) -> Result<TokenStream> {
    self.emit(st)
  }
}

static GENERATORS: [&dyn CodeGenerator; 2] = [&InputCG, &ConstCG];

/// Emits unchanged struct with the nine default derives.
/// Deriving PartialEq,Eq also gives us StructuralPartialEq.
struct InputCG;
impl CodeGenerator for InputCG {
  fn emit_float(
    &self,
    StrongType { input, .. }: &StrongType,
  ) -> Result<TokenStream> {
    Ok(quote! {
      #[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
      #[repr(transparent)]
      #input
    })
  }

  fn emit(&self, StrongType { input, .. }: &StrongType) -> Result<TokenStream> {
    Ok(quote! {
      #[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
      #[repr(transparent)]
      #input
    })
  }
}

struct ConstCG;
impl CodeGenerator for ConstCG {
  fn emit_int(
    &self,
    StrongType { outer, outer_vis, inner, .. }: &StrongType,
  ) -> Result<TokenStream> {
    Ok(quote! {
      impl #outer {
        #outer_vis const MIN: Self = Self(#inner::MIN);
        #outer_vis const MAX: Self = Self(#inner::MAX);
        #outer_vis const BITS: u32 = #inner::BITS;

        #outer_vis const ZERO: Self = Self(0);
        #outer_vis const ONE: Self = Self(1);
      }
    })
  }

  fn emit_float(
    &self,
    StrongType { outer, outer_vis, inner, .. }: &StrongType,
  ) -> Result<TokenStream> {
    Ok(quote! {
      impl #outer {
        #outer_vis const MIN: Self = Self(#inner::MIN);
        #outer_vis const MAX: Self = Self(#inner::MAX);

        #outer_vis const ZERO: Self = Self(0.);
        #outer_vis const ONE: Self = Self(1.);
      }
    })
  }

  fn emit_bool(
    &self,
    StrongType { outer, outer_vis, .. }: &StrongType,
  ) -> Result<TokenStream> {
    Ok(quote! {
      impl #outer {
        #outer_vis const TRUE: Self = Self(true);
        #outer_vis const FALSE: Self = Self(false);
      }
    })
  }

  fn emit(&self, StrongType { .. }: &StrongType) -> Result<TokenStream> {
    Ok(TokenStream::default())
  }
}

impl StrongType {
  fn emit_impl(&self, output: &mut TokenStream) -> Result<()> {
    let Self { outer, outer_vis, inner, inner_base, .. } = self;
    let inner_parse_err = inner_base.parse_err_tokens();

    output.extend(quote! {
      impl #outer {
        #[must_use]
        #[inline(always)]
        #outer_vis const fn new(inner: #inner) -> Self { Self(inner) }

        #[must_use]
        #[inline(always)]
        #outer_vis const fn into_inner(self) -> #inner { self.0 }
      }

      impl ::core::fmt::Display for #outer {
        #[inline(always)]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
        { ::core::fmt::Display::fmt(&self.0, f) }
      }

      impl ::core::str::FromStr for #outer {
        type Err = #inner_parse_err;
        #[inline(always)]
        fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
          #inner::from_str(s).map(Self)
        }
      }

      // TODO: macro flag param
      impl ::core::convert::From<#inner> for #outer {
        #[must_use]
        #[inline(always)]
        fn from(inner: #inner) -> Self { Self(inner) }
      }

      // TODO: macro flag param
      impl ::core::convert::From<#outer> for #inner {
        #[must_use]
        #[inline(always)]
        fn from(outer: #outer) -> Self { outer.0 }
      }

      // TODO: macro flag param
      impl ::core::borrow::Borrow<#inner> for #outer {
        #[must_use]
        #[inline(always)]
        fn borrow(&self) -> &#inner { &self.0 }
      }
    });
    Ok(())
  }

  fn emit_impl_ops_alg(&self, output: &mut TokenStream) -> Result<()> {
    let Self { outer, .. } = self;
    output.extend(quote! {
      impl ::core::ops::Add for #outer {
        type Output = Self;
        #[must_use]
        #[inline(always)]
        fn add(self, other: Self) -> Self::Output { Self(self.0.add(other.0)) }
      }

      impl ::core::ops::AddAssign for #outer {
        #[inline(always)]
        fn add_assign(&mut self, other: Self) { self.0.add_assign(other.0); }
      }

      impl ::core::ops::Sub for #outer {
        type Output = Self;
        #[must_use]
        #[inline(always)]
        fn sub(self, other: Self) -> Self::Output { Self(self.0.sub(other.0)) }
      }

      impl ::core::ops::SubAssign for #outer {
        #[inline(always)]
        fn sub_assign(&mut self, other: Self) { self.0.sub_assign(other.0); }
      }

      impl ::core::ops::Mul for #outer {
        type Output = Self;
        #[must_use]
        #[inline(always)]
        fn mul(self, other: Self) -> Self::Output { Self(self.0.mul(other.0)) }
      }

      impl ::core::ops::MulAssign for #outer {
        #[inline(always)]
        fn mul_assign(&mut self, other: Self) { self.0.mul_assign(other.0); }
      }

      impl ::core::ops::Div for #outer {
        type Output = Self;
        #[must_use]
        #[inline(always)]
        fn div(self, other: Self) -> Self::Output { Self(self.0.div(other.0)) }
      }

      impl ::core::ops::DivAssign for #outer {
        #[inline(always)]
        fn div_assign(&mut self, other: Self) { self.0.div_assign(other.0); }
      }

      impl ::core::ops::Rem for #outer {
        type Output = Self;
        #[inline(always)]
        fn rem(self, other: Self) -> Self::Output { Self(self.0.rem(other.0)) }
      }

      impl ::core::ops::RemAssign for #outer {
        #[inline(always)]
        fn rem_assign(&mut self, other: Self) { self.0.rem_assign(other.0); }
      }
    });
    Ok(())
  }

  fn emit_impl_ops_bit(&self, output: &mut TokenStream) -> Result<()> {
    let Self { outer, .. } = self;
    output.extend(quote! {
      impl ::core::ops::BitAnd for #outer {
        type Output = Self;
        #[must_use]
        #[inline(always)]
        fn bitand(self, other: Self) -> Self::Output {
          Self(self.0.bitand(other.0))
        }
      }

      impl ::core::ops::BitAndAssign for #outer {
        #[inline(always)]
        fn bitand_assign(&mut self, other: Self) {
          self.0.bitand_assign(other.0);
        }
      }

      impl ::core::ops::BitOr for #outer {
        type Output = Self;
        #[must_use]
        #[inline(always)]
        fn bitor(self, other: Self) -> Self::Output {
          Self(self.0.bitor(other.0))
        }
      }

      impl ::core::ops::BitOrAssign for #outer {
        #[inline(always)]
        fn bitor_assign(&mut self, other: Self) {
          self.0.bitor_assign(other.0);
        }
      }

      impl ::core::ops::BitXor for #outer {
        type Output = Self;
        #[must_use]
        #[inline(always)]
        fn bitxor(self, other: Self) -> Self::Output {
          Self(self.0.bitxor(other.0))
        }
      }

      impl ::core::ops::BitXorAssign for #outer {
        #[inline(always)]
        fn bitxor_assign(&mut self, other: Self) {
          self.0.bitxor_assign(other.0);
        }
      }

      impl ::core::ops::Not for #outer {
        type Output = Self;
        #[must_use]
        #[inline(always)]
        fn not(self) -> Self::Output { Self(self.0.not()) }
      }
    });
    Ok(())
  }

  fn emit_impl_int_display(&self, output: &mut TokenStream) -> Result<()> {
    let Self { outer, .. } = self;
    output.extend(quote! {
      impl ::core::fmt::Binary for #outer {
        #[inline(always)]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          ::core::fmt::Binary::fmt(&self.0, f)
        }
      }

      impl ::core::fmt::Octal for #outer {
        #[inline(always)]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          ::core::fmt::Octal::fmt(&self.0, f)
        }
      }

      impl ::core::fmt::LowerHex for #outer {
        #[inline(always)]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          ::core::fmt::LowerHex::fmt(&self.0, f)
        }
      }

      impl ::core::fmt::UpperHex for #outer {
        #[inline(always)]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          ::core::fmt::UpperHex::fmt(&self.0, f)
        }
      }
    });
    Ok(())
  }

  // fn emit_impl_XXX(&self, output: &mut TokenStream) -> Result<()> {
  //   let Self { outer, inner, .. } = self;
  //   output.extend(quote! {
  //     // XXX
  //   });
  //   Ok(())
  // }

  fn emit_impl_serde(&self, output: &mut TokenStream) -> Result<()> {
    let Self { outer, .. } = self;
    output.extend(quote! {
      impl ::serde::Serialize for #outer {
        #[inline(always)]
        fn serialize<S: ::serde::Serializer>(&self, s: S)
          -> ::core::result::Result<S::Ok, S::Error>
        { ::serde::Serialize::serialize(&self.0, s) }
      }
      impl<'de> ::serde::Deserialize<'de> for #outer {
        #[inline(always)]
        fn deserialize<D: ::serde::Deserializer<'de>>(d: D)
          -> ::core::result::Result<Self, D::Error>
        { ::serde::Deserialize::deserialize(d).map(Self) }
      }
    });
    Ok(())
  }
}
