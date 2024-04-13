macro_rules! types {
  (  $($inner:ident)+ ) => {
    mod types {
      $(
        paste::paste! {
          #[::strongly::typed($inner)]
          pub struct [<Strong $inner>];
        }
      )+
    }
  };
}

types!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 char bool);

macro_rules! tests {
  ( $($inner:ident)+ ) => {
    $(
      ::paste::paste! {
        #[test]
        fn [<test_size_align_ $inner>]() {
          use types::[<Strong $inner>] as Strong;
          ::core::assert_eq!(
            ::core::mem::size_of::<$inner>(),
            ::core::mem::size_of::<Strong>(),
          );
          ::core::assert_eq!(
            ::core::mem::align_of::<$inner>(),
            ::core::mem::align_of::<Strong>(),
          );
        }
      }
    )+
  }
}

tests!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 char bool);
