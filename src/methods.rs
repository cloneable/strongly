use crate::{CodeGenerator, StrongType};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use syn::Result;

pub struct MethodsCG;

impl CodeGenerator for MethodsCG {
  fn emit_float(
    &self,
    StrongType { outer, outer_vis, inner, .. }: &StrongType,
  ) -> Result<TokenStream> {
    let (bytesize, bitstype) = match inner.to_string().as_str() {
      "f32" => {
        (Literal::usize_suffixed(4), Ident::new("u32", Span::call_site()))
      }
      "f64" => {
        (Literal::usize_suffixed(8), Ident::new("u64", Span::call_site()))
      }
      _ => panic!("unexpected type: {}", inner),
    };
    Ok(quote! {
      impl #outer {
        #[must_use]
        #[inline(always)]
        #outer_vis fn is_nan(self) -> bool { self.0.is_nan() }
        #[must_use]
        #[inline(always)]
        #outer_vis fn is_infinite(self) -> bool { self.0.is_infinite() }
        #[must_use]
        #[inline(always)]
        #outer_vis fn is_finite(self) -> bool { self.0.is_finite() }
        #[must_use]
        #[inline(always)]
        #outer_vis fn is_subnormal(self) -> bool { self.0.is_subnormal() }
        #[must_use]
        #[inline(always)]
        #outer_vis fn is_normal(self) -> bool { self.0.is_normal() }
        #[must_use]
        #[inline(always)]
        #outer_vis fn classify(self) -> ::core::num::FpCategory { self.0.classify() }
        #[must_use]
        #[inline(always)]
        #outer_vis fn is_sign_positive(self) -> bool { self.0.is_sign_positive() }
        #[must_use]
        #[inline(always)]
        #outer_vis fn is_sign_negative(self) -> bool { self.0.is_sign_negative() }
        #[must_use]
        #[inline(always)]
        #outer_vis fn recip(self) -> Self { Self(self.0.recip()) }
        #[must_use]
        #[inline(always)]
        #outer_vis fn to_degrees(self) -> Self { Self(self.0.to_degrees()) }
        #[must_use]
        #[inline(always)]
        #outer_vis fn to_radians(self) -> Self { Self(self.0.to_radians()) }
        #[must_use]
        #[inline(always)]
        #outer_vis fn max(self, other: Self) -> Self { Self(self.0.max(other.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis fn min(self, other: Self) -> Self { Self(self.0.min(other.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis fn to_bits(self) -> #bitstype { self.0.to_bits() }
        #[must_use]
        #[inline(always)]
        #outer_vis fn from_bits(v: #bitstype) -> Self { Self(#inner::from_bits(v)) }
        #[must_use]
        #[inline(always)]
        #outer_vis fn to_be_bytes(self) -> [u8; #bytesize] { self.0.to_be_bytes() }
        #[must_use]
        #[inline(always)]
        #outer_vis fn to_le_bytes(self) -> [u8; #bytesize] { self.0.to_le_bytes() }
        #[must_use]
        #[inline(always)]
        #outer_vis fn to_ne_bytes(self) -> [u8; #bytesize] { self.0.to_ne_bytes() }
        #[must_use]
        #[inline(always)]
        #outer_vis fn from_be_bytes(bytes: [u8; #bytesize]) -> Self { Self(#inner::from_be_bytes(bytes)) }
        #[must_use]
        #[inline(always)]
        #outer_vis fn from_le_bytes(bytes: [u8; #bytesize]) -> Self { Self(#inner::from_le_bytes(bytes)) }
        #[must_use]
        #[inline(always)]
        #outer_vis fn from_ne_bytes(bytes: [u8; #bytesize]) -> Self { Self(#inner::from_ne_bytes(bytes)) }
        #[must_use]
        #[inline(always)]
        #outer_vis fn total_cmp(&self, other: &Self) -> ::core::cmp::Ordering { self.0.total_cmp(&other.0) }
        #[must_use]
        #[inline(always)]
        #outer_vis fn clamp(self, min: Self, max: Self) -> Self { Self(self.0.clamp(min.0, max.0)) }
      }
    })
  }

  fn emit_bool(
    &self,
    StrongType { outer, outer_vis, .. }: &StrongType,
  ) -> Result<TokenStream> {
    Ok(quote! {
      impl #outer {
        #[must_use]
        #[inline(always)]
        #outer_vis fn then<T, F: ::core::ops::FnOnce() -> T>(self, f: F) -> ::core::option::Option<T> { self.0.then(f) }
        #[must_use]
        #[inline(always)]
        #outer_vis fn then_some<T>(self, t: T) -> ::core::option::Option<T> { self.0.then_some(t) }
      }
    })
  }
}
