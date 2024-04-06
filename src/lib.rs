use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
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
  if !params.is_empty() {
    return Err(Error::new(
      Span::call_site(),
      String::from("This macro does not (yet) accept parameters"),
    ));
  }

  let item = syn::parse2::<ItemStruct>(input.clone())?;
  let StrongType { outer, outer_vis, inner, .. } = (&item).try_into()?;

  let mut output = TokenStream::new();

  // Emit unchanged struct with the nine default derives.
  // Deriving PartialEq,Eq also gives us StructuralPartialEq.
  output.extend(quote! {
    #[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
  });
  output.extend(input);

  // TODO: serde macro param
  if cfg!(feature = "serde") {
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
  }

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
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.0, f)
      }
    }

    // TODO: FromStr

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

  Ok(output)
}

struct StrongType<'a> {
  outer: &'a Ident,
  outer_vis: &'a Visibility,
  inner: &'a Ident,
  inner_vis: &'a Visibility,
}

impl<'a> TryFrom<&'a ItemStruct> for StrongType<'a> {
  type Error = Error;

  fn try_from(item: &'a ItemStruct) -> Result<Self> {
    let outer = &item.ident;
    let outer_vis = &item.vis;

    let Fields::Unnamed(fields) = &item.fields else {
      return Err(Error::new(item.span(), "not tuple struct"));
    };
    if fields.unnamed.len() != 1 {
      return Err(Error::new(item.span(), "not newtype struct"));
    }
    let field = fields.unnamed.first().expect("first element");
    let inner = match &field.ty {
      Type::Path(path) => path.path.require_ident()?,
      _ => return Err(Error::new(field.ty.span(), "unexpected type")),
    };
    let inner_vis = &field.vis;

    Ok(StrongType { outer, outer_vis, inner, inner_vis })
  }
}
