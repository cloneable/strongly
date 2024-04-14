use crate::{CodeGenerator, StrongType};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Result;

pub struct ShiftOpsCG;

impl CodeGenerator for ShiftOpsCG {
  fn emit_int(
    &self,
    StrongType { outer, .. }: &StrongType,
  ) -> Result<TokenStream> {
    Ok(quote! {
      macro_rules! shift_ops {
        ( $($prim:ident)+ ) => {
          $(
            impl ::core::ops::Shl<$prim> for #outer {
              type Output = #outer;
              #[must_use]
              #[inline(always)]
              fn shl(self, rhs: $prim) -> Self::Output { #outer(self.0.shl(rhs)) }
            }
            impl ::core::ops::Shl<&$prim> for #outer {
              type Output = #outer;
              #[must_use]
              #[inline(always)]
              fn shl(self, rhs: &$prim) -> Self::Output { #outer(self.0.shl(rhs)) }
            }
            impl ::core::ops::Shl<$prim> for &#outer {
              type Output = #outer;
              #[must_use]
              #[inline(always)]
              fn shl(self, rhs: $prim) -> Self::Output { #outer(self.0.shl(rhs)) }
            }
            impl ::core::ops::Shl<&$prim> for &#outer {
              type Output = #outer;
              #[must_use]
              #[inline(always)]
              fn shl(self, rhs: &$prim) -> Self::Output { #outer(self.0.shl(rhs)) }
            }
            impl ::core::ops::ShlAssign<$prim> for #outer {
              #[inline(always)]
              fn shl_assign(&mut self, rhs: $prim) { self.0.shl_assign(rhs) }
            }
            impl ::core::ops::ShlAssign<&$prim> for #outer {
              #[inline(always)]
              fn shl_assign(&mut self, rhs: &$prim) { self.0.shl_assign(rhs) }
            }
            impl ::core::ops::Shr<$prim> for #outer {
              type Output = #outer;
              #[must_use]
              #[inline(always)]
              fn shr(self, rhs: $prim) -> Self::Output { #outer(self.0.shr(rhs)) }
            }
            impl ::core::ops::Shr<&$prim> for #outer {
              type Output = #outer;
              #[must_use]
              #[inline(always)]
              fn shr(self, rhs: &$prim) -> Self::Output { #outer(self.0.shr(rhs)) }
            }
            impl ::core::ops::Shr<$prim> for &#outer {
              type Output = #outer;
              #[must_use]
              #[inline(always)]
              fn shr(self, rhs: $prim) -> Self::Output { #outer(self.0.shr(rhs)) }
            }
            impl ::core::ops::Shr<&$prim> for &#outer {
              type Output = #outer;
              #[must_use]
              #[inline(always)]
              fn shr(self, rhs: &$prim) -> Self::Output { #outer(self.0.shr(rhs)) }
            }
            impl ::core::ops::ShrAssign<$prim> for #outer {
              #[inline(always)]
              fn shr_assign(&mut self, rhs: $prim) { self.0.shr_assign(rhs) }
            }
            impl ::core::ops::ShrAssign<&$prim> for #outer {
              #[inline(always)]
              fn shr_assign(&mut self, rhs: &$prim) { self.0.shr_assign(rhs) }
            }
          )+
        };
      }
      shift_ops!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
    })
  }
}
