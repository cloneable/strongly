//! Compilation smoke test

#![no_std]
#![no_implicit_prelude]
#![deny(
  unsafe_code,
  deprecated,
  rust_2018_idioms,
  future_incompatible,
  clippy::all,
  clippy::pedantic,
  clippy::nursery
)]

#[::strongly::typed(i8)]
pub struct StronglyTypedI8;

#[::strongly::typed(i16)]
pub struct StronglyTypedI16;

#[::strongly::typed(i32)]
pub struct StronglyTypedI32;

#[::strongly::typed(i64)]
pub struct StronglyTypedI64;

#[::strongly::typed(i128)]
pub struct StronglyTypedI128;

#[::strongly::typed(u8)]
pub struct StronglyTypedU8;

#[::strongly::typed(u16)]
pub struct StronglyTypedU16;

#[::strongly::typed(u32)]
pub struct StronglyTypedU32;

#[::strongly::typed(u64)]
pub struct StronglyTypedU64;

#[::strongly::typed(u128)]
pub struct StronglyTypedU128;

#[::strongly::typed(f32)]
pub struct StronglyTypedF32;

#[::strongly::typed(f64)]
pub struct StronglyTypedF64;

#[::strongly::typed(isize)]
pub struct StronglyTypedIsize;

#[::strongly::typed(usize)]
pub struct StronglyTypedUsize;

#[::strongly::typed(char)]
pub struct StronglyTypedChar;

#[::strongly::typed(bool)]
pub struct StronglyTypedBool;

extern "C" {
  pub fn pass(v: StronglyTypedI8) -> StronglyTypedI8;
}

#[repr(C)]
pub struct FfiStruct {
  field: StronglyTypedU32,
}

const _: () = {
  use ::core::clone::Clone;
  use ::core::cmp::{Eq, Ord, PartialEq, PartialOrd};
  use ::core::default::Default;
  use ::core::fmt::{
    Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex,
  };
  use ::core::hash::Hash;
  use ::core::marker::{Copy, Send, Sized, Sync, Unpin};
  use ::core::panic::{RefUnwindSafe, UnwindSafe};
  use ::core::str::FromStr;
  use ::static_assertions::assert_impl_all;

  macro_rules! assert_int_traits {
    ( $($outer:ty),* ) => {
      $(
        assert_impl_all!($outer: Sized, Send, Sync, Unpin, UnwindSafe, RefUnwindSafe);
        assert_impl_all!($outer: Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash);
        assert_impl_all!($outer: Display, FromStr, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp);
      )*
    };
  }

  macro_rules! assert_float_traits {
    ( $($outer:ty),* ) => {
      $(
        assert_impl_all!($outer: Sized, Send, Sync, Unpin, UnwindSafe, RefUnwindSafe);
        assert_impl_all!($outer: Copy, Clone, Default, Debug, PartialEq, PartialOrd);
        assert_impl_all!($outer: Display, FromStr);
      )*
    };
  }

  macro_rules! assert_special_traits {
    ( $($outer:ty),* ) => {
      $(
        assert_impl_all!($outer: Sized, Send, Sync, Unpin, UnwindSafe, RefUnwindSafe);
        assert_impl_all!($outer: Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash);
        assert_impl_all!($outer: Display, FromStr);
      )*
    };
  }

  assert_int_traits!(
    StronglyTypedI8,
    StronglyTypedI16,
    StronglyTypedI32,
    StronglyTypedI64,
    StronglyTypedI128,
    StronglyTypedIsize,
    StronglyTypedU8,
    StronglyTypedU16,
    StronglyTypedU32,
    StronglyTypedU64,
    StronglyTypedU128,
    StronglyTypedUsize
  );
  assert_float_traits!(StronglyTypedF32, StronglyTypedF64);
  assert_special_traits!(StronglyTypedChar, StronglyTypedBool);
};
