use crate::{CodeGenerator, Params, StrongType};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use syn::Result;

pub struct MethodsCG;

impl CodeGenerator for MethodsCG {
  fn emit_int(
    &self,
    _: &Params,
    StrongType { outer, outer_vis, inner, inner_base, .. }: &StrongType,
  ) -> Result<TokenStream> {
    let (bytesize, unsigned) = match inner.to_string().as_str() {
      "i8" | "u8" => {
        (Literal::usize_suffixed(1), Ident::new("u8", Span::call_site()))
      }
      "i16" | "u16" => {
        (Literal::usize_suffixed(2), Ident::new("u16", Span::call_site()))
      }
      "i32" | "u32" => {
        (Literal::usize_suffixed(4), Ident::new("u32", Span::call_site()))
      }
      "i64" | "u64" => {
        (Literal::usize_suffixed(8), Ident::new("u64", Span::call_site()))
      }
      "i128" | "u128" => {
        (Literal::usize_suffixed(16), Ident::new("u128", Span::call_site()))
      }
      "isize" | "usize" => {
        (Literal::usize_suffixed(8), Ident::new("usize", Span::call_site()))
      }
      _ => panic!("unexpected type: {}", inner),
    };
    let inner_parse_err = inner_base.parse_err_tokens();
    Ok(quote! {
      impl #outer {
        #[inline(always)]
        #outer_vis fn from_str_radix(src: &str, radix: u32) -> ::core::result::Result<#outer, #inner_parse_err> {
          #inner::from_str_radix(src, radix).map(Self)
        }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn count_ones(self) -> u32 { self.0.count_ones() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn count_zeros(self) -> u32 { self.0.count_zeros() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn leading_zeros(self) -> u32 { self.0.leading_zeros() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn trailing_zeros(self) -> u32 { self.0.trailing_zeros() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn leading_ones(self) -> u32 { self.0.leading_ones() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn trailing_ones(self) -> u32 { self.0.trailing_ones() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn rotate_left(self, n: u32) -> Self { Self(self.0.rotate_left(n)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn rotate_right(self, n: u32) -> Self { Self(self.0.rotate_right(n)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn swap_bytes(self) -> Self { Self(self.0.swap_bytes()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn reverse_bits(self) -> Self { Self(self.0.reverse_bits()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn from_be(x: Self) -> Self { Self(#inner::from_be(x.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn from_le(x: Self) -> Self { Self(#inner::from_le(x.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn to_be(self) -> Self { Self(self.0.to_be()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn to_le(self) -> Self { Self(self.0.to_le()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_add(self, rhs: Self) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_add(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_sub(self, rhs: Self) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_sub(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_mul(self, rhs: Self) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_mul(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_div(self, rhs: Self) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_div(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_div_euclid(self, rhs: Self) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_div_euclid(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_rem(self, rhs: Self) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_rem(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_rem_euclid(self, rhs: Self) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_rem_euclid(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_shl(self, rhs: u32) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_shl(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_shr(self, rhs: u32) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_shr(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_pow(self, exp: u32) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_pow(exp)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn saturating_add(self, rhs: Self) -> Self { Self(self.0.saturating_add(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn saturating_sub(self, rhs: Self) -> Self { Self(self.0.saturating_sub(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn saturating_mul(self, rhs: Self) -> Self { Self(self.0.saturating_mul(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn saturating_div(self, rhs: Self) -> Self { Self(self.0.saturating_div(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn saturating_pow(self, exp: u32) -> Self { Self(self.0.saturating_pow(exp)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_add(self, rhs: Self) -> Self { Self(self.0.wrapping_add(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_sub(self, rhs: Self) -> Self { Self(self.0.wrapping_sub(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_mul(self, rhs: Self) -> Self { Self(self.0.wrapping_mul(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_div(self, rhs: Self) -> Self { Self(self.0.wrapping_div(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_div_euclid(self, rhs: Self) -> Self { Self(self.0.wrapping_div_euclid(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_rem(self, rhs: Self) -> Self { Self(self.0.wrapping_rem(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_rem_euclid(self, rhs: Self) -> Self { Self(self.0.wrapping_rem_euclid(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_shl(self, rhs: u32) -> Self { Self(self.0.wrapping_shl(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_shr(self, rhs: u32) -> Self { Self(self.0.wrapping_shr(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_pow(self, exp: u32) -> Self { Self(self.0.wrapping_pow(exp)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_add(self, rhs: Self) -> (Self, bool) { Self::map_of_res(self.0.overflowing_add(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_sub(self, rhs: Self) -> (Self, bool) { Self::map_of_res(self.0.overflowing_sub(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_mul(self, rhs: Self) -> (Self, bool) { Self::map_of_res(self.0.overflowing_mul(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_div(self, rhs: Self) -> (Self, bool) { Self::map_of_res(self.0.overflowing_div(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) { Self::map_of_res(self.0.overflowing_div_euclid(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_rem(self, rhs: Self) -> (Self, bool) { Self::map_of_res(self.0.overflowing_rem(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) { Self::map_of_res(self.0.overflowing_rem_euclid(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_shl(self, rhs: u32) -> (Self, bool) { Self::map_of_res(self.0.overflowing_shl(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_shr(self, rhs: u32) -> (Self, bool) { Self::map_of_res(self.0.overflowing_shr(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_pow(self, exp: u32) -> (Self, bool) { Self::map_of_res(self.0.overflowing_pow(exp)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn pow(self, exp: u32) -> Self { Self(self.0.pow(exp)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn div_euclid(self, rhs: Self) -> Self { Self(self.0.div_euclid(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn rem_euclid(self, rhs: Self) -> Self { Self(self.0.rem_euclid(rhs.0)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn ilog(self, base: Self) -> u32 { self.0.ilog(base.0) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn ilog2(self) -> u32 { self.0.ilog2() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn ilog10(self) -> u32 { self.0.ilog10() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_ilog(self, base: Self) -> ::core::option::Option<u32> { self.0.checked_ilog(base.0) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_ilog2(self) -> ::core::option::Option<u32> { self.0.checked_ilog2() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_ilog10(self) -> ::core::option::Option<u32> { self.0.checked_ilog10() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn to_be_bytes(self) -> [u8; #bytesize] { self.0.to_be_bytes() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn to_le_bytes(self) -> [u8; #bytesize] { self.0.to_le_bytes() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn to_ne_bytes(self) -> [u8; #bytesize] { self.0.to_ne_bytes() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn from_be_bytes(bytes: [u8; #bytesize]) -> Self { Self(#inner::from_be_bytes(bytes)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn from_le_bytes(bytes: [u8; #bytesize]) -> Self { Self(#inner::from_le_bytes(bytes)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn from_ne_bytes(bytes: [u8; #bytesize]) -> Self { Self(#inner::from_ne_bytes(bytes)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn abs_diff(self, other: Self) -> #unsigned { self.0.abs_diff(other.0) }

        #[inline(always)]
        const fn map_of_res(result: (#inner, bool)) -> (#outer, bool) { (#outer(result.0), result.1) }
        #[inline(always)]
        const fn map_self(option: ::core::option::Option<#inner>) -> ::core::option::Option<#outer> {
          use ::core::option::Option::{Some, None};
          match option {
            Some(inner) => Some(#outer(inner)),
            None => None,
          }
        }

        #[must_use]
        #[inline(always)]
        #outer_vis const fn range(self, end: Self) -> impl ::core::iter::Iterator<Item = Self> {
          struct Iter {
            start: <Self as ::core::iter::Iterator>::Item,
            end: <Self as ::core::iter::Iterator>::Item,
          }
          impl ::core::iter::Iterator for Iter {
            type Item = #outer;
            #[inline(always)]
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
              if self.start < self.end {
                let next = self.start;
                self.start += Self::Item::ONE;
                ::core::option::Option::Some(next)
              } else {
                ::core::option::Option::None
              }
            }
          }
          Iter { start: self, end }
        }

        #[must_use]
        #[inline(always)]
        #outer_vis const fn range_incl(self, end: Self) -> impl ::core::iter::Iterator<Item = Self> {
          struct Iter {
            start: <Self as ::core::iter::Iterator>::Item,
            end: <Self as ::core::iter::Iterator>::Item,
          }
          impl ::core::iter::Iterator for Iter {
            type Item = #outer;
            #[inline(always)]
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
              if self.start <= self.end {
                let next = self.start;
                self.start += Self::Item::ONE;
                ::core::option::Option::Some(next)
              } else {
                ::core::option::Option::None
              }
            }
          }
          Iter { start: self, end }
        }
      }
    })
  }

  fn emit_signed_int(
    &self,
    params: &Params,
    st: &StrongType,
  ) -> Result<TokenStream> {
    let mut output = self.emit_int(params, st)?;

    let StrongType { outer, outer_vis, inner, .. } = st;
    let unsigned = match inner.to_string().as_str() {
      "i8" => Ident::new("u8", Span::call_site()),
      "i16" => Ident::new("u16", Span::call_site()),
      "i32" => Ident::new("u32", Span::call_site()),
      "i64" => Ident::new("u64", Span::call_site()),
      "i128" => Ident::new("u128", Span::call_site()),
      "isize" => Ident::new("usize", Span::call_site()),
      _ => panic!("unexpected type: {}", inner),
    };

    output.extend(quote! {
      impl #outer {
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_add_unsigned(self, rhs: #unsigned) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_add_unsigned(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_sub_unsigned(self, rhs: #unsigned) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_sub_unsigned(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_neg(self) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_neg()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn checked_abs(self) -> ::core::option::Option<Self> { Self::map_self(self.0.checked_abs()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn saturating_add_unsigned(self, rhs: #unsigned) -> Self { Self(self.0.saturating_add_unsigned(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn saturating_sub_unsigned(self, rhs: #unsigned) -> Self { Self(self.0.saturating_sub_unsigned(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn saturating_neg(self) -> Self { Self(self.0.saturating_neg()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn saturating_abs(self) -> Self { Self(self.0.saturating_abs()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_add_unsigned(self, rhs: #unsigned) -> Self { Self(self.0.wrapping_add_unsigned(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_sub_unsigned(self, rhs: #unsigned) -> Self { Self(self.0.wrapping_sub_unsigned(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_neg(self) -> Self { Self(self.0.wrapping_neg()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn wrapping_abs(self) -> Self { Self(self.0.wrapping_abs()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn unsigned_abs(self) -> #unsigned { self.0.unsigned_abs() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_add_unsigned(self, rhs: #unsigned) -> (Self, bool) { Self::map_of_res(self.0.overflowing_add_unsigned(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_sub_unsigned(self, rhs: #unsigned) -> (Self, bool) { Self::map_of_res(self.0.overflowing_sub_unsigned(rhs)) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_neg(self) -> (Self, bool) { Self::map_of_res(self.0.overflowing_neg()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn overflowing_abs(self) -> (Self, bool) { Self::map_of_res(self.0.overflowing_abs()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn abs(self) -> Self { Self(self.0.abs()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn signum(self) -> Self { Self(self.0.signum()) }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn is_positive(self) -> bool { self.0.is_positive() }
        #[must_use]
        #[inline(always)]
        #outer_vis const fn is_negative(self) -> bool { self.0.is_negative() }
      }
    });

    Ok(output)
  }

  fn emit_float(
    &self,
    _: &Params,
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
    _: &Params,
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
