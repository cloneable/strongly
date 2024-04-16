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

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedI8(i8);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedI16(i16);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedI32(i32);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedI64(i64);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedI128(i128);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedU8(u8);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedU16(u16);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedU32(u32);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedU64(u64);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedU128(u128);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedF32(f32);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedF64(f64);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedIsize(isize);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedUsize(usize);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedChar(char);

#[::strongly::typed(convert, deref, serde)]
pub struct StronglyTypedBool(bool);

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
