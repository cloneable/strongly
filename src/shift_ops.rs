use crate::{CodeGenerator, Params, StrongType};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Result;

pub struct ShiftOpsCG;

impl CodeGenerator for ShiftOpsCG {
  fn emit_int(
    &self,
    _: &Params,
    StrongType { outer, .. }: &StrongType,
  ) -> Result<TokenStream> {
    let int_prims = [
      "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
      "u128", "usize",
    ]
    .map(|s| Ident::new(s, Span::call_site()));

    let mut output = TokenStream::new();
    for prim in int_prims {
      output.extend(template_shl(&prim, outer)?);
      output.extend(template_shl_assign(&prim, outer)?);
      output.extend(template_shr(&prim, outer)?);
      output.extend(template_shr_assign(&prim, outer)?);
    }

    Ok(output)
  }
}

fn template_shl(prim: &Ident, outer: &Ident) -> Result<TokenStream> {
  Ok(quote! {
    impl ::core::ops::Shl<#prim> for #outer {
      type Output = #outer;
      #[must_use]
      #[inline(always)]
      fn shl(self, rhs: #prim) -> Self::Output { #outer(self.0.shl(rhs)) }
    }
    impl ::core::ops::Shl<&#prim> for #outer {
      type Output = #outer;
      #[must_use]
      #[inline(always)]
      fn shl(self, rhs: &#prim) -> Self::Output { #outer(self.0.shl(rhs)) }
    }
    impl ::core::ops::Shl<#prim> for &#outer {
      type Output = #outer;
      #[must_use]
      #[inline(always)]
      fn shl(self, rhs: #prim) -> Self::Output { #outer(self.0.shl(rhs)) }
    }
    impl ::core::ops::Shl<&#prim> for &#outer {
      type Output = #outer;
      #[must_use]
      #[inline(always)]
      fn shl(self, rhs: &#prim) -> Self::Output { #outer(self.0.shl(rhs)) }
    }
  })
}

fn template_shl_assign(prim: &Ident, outer: &Ident) -> Result<TokenStream> {
  Ok(quote! {
    impl ::core::ops::ShlAssign<#prim> for #outer {
      #[inline(always)]
      fn shl_assign(&mut self, rhs: #prim) { self.0.shl_assign(rhs) }
    }
    impl ::core::ops::ShlAssign<&#prim> for #outer {
      #[inline(always)]
      fn shl_assign(&mut self, rhs: &#prim) { self.0.shl_assign(rhs) }
    }
  })
}

fn template_shr(prim: &Ident, outer: &Ident) -> Result<TokenStream> {
  Ok(quote! {
    impl ::core::ops::Shr<#prim> for #outer {
      type Output = #outer;
      #[must_use]
      #[inline(always)]
      fn shr(self, rhs: #prim) -> Self::Output { #outer(self.0.shr(rhs)) }
    }
    impl ::core::ops::Shr<&#prim> for #outer {
      type Output = #outer;
      #[must_use]
      #[inline(always)]
      fn shr(self, rhs: &#prim) -> Self::Output { #outer(self.0.shr(rhs)) }
    }
    impl ::core::ops::Shr<#prim> for &#outer {
      type Output = #outer;
      #[must_use]
      #[inline(always)]
      fn shr(self, rhs: #prim) -> Self::Output { #outer(self.0.shr(rhs)) }
    }
    impl ::core::ops::Shr<&#prim> for &#outer {
      type Output = #outer;
      #[must_use]
      #[inline(always)]
      fn shr(self, rhs: &#prim) -> Self::Output { #outer(self.0.shr(rhs)) }
    }
  })
}

fn template_shr_assign(prim: &Ident, outer: &Ident) -> Result<TokenStream> {
  Ok(quote! {
    impl ::core::ops::ShrAssign<#prim> for #outer {
      #[inline(always)]
      fn shr_assign(&mut self, rhs: #prim) { self.0.shr_assign(rhs) }
    }
    impl ::core::ops::ShrAssign<&#prim> for #outer {
      #[inline(always)]
      fn shr_assign(&mut self, rhs: &#prim) { self.0.shr_assign(rhs) }
    }
  })
}
