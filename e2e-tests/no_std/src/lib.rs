//! Compilation smoke test

#![no_std]
#![no_implicit_prelude]

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
