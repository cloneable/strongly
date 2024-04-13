macro_rules! types {
  (  $($inner:ident)+ ) => {
    $(
      paste::paste! {
        #[::strongly::typed($inner)]
        pub struct [<Strong $inner>];
      }
    )+
  };
}

types!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 char bool);

macro_rules! tests {
  ( $($inner:ident)+ ) => {
    $(
      ::paste::paste! {
        #[test]
        fn [<test_size_align_ $inner>]() {
          use $inner as Inner;
          use [<Strong $inner>] as Outer;
          ::core::assert_eq!(
            ::core::mem::size_of::<Inner>(),
            ::core::mem::size_of::<Outer>(),
          );
          ::core::assert_eq!(
            ::core::mem::align_of::<Inner>(),
            ::core::mem::align_of::<Outer>(),
          );
        }

        #[test]
        fn [<test_default_ $inner>]() {
          use $inner as Inner;
          use [<Strong $inner>] as Outer;
          ::core::assert_eq!(
            Inner::default(),
            Outer::default().0,
          );
        }

        #[test]
        fn [<test_new_ $inner>]() {
          use $inner as Inner;
          use [<Strong $inner>] as Outer;
          ::core::assert_eq!(
            Inner::default(),
            Outer::new(Inner::default()).0
          );
        }

        // TODO: Debug
        // TODO: Display, Binary, Octal, Lower/UpperHex, Lower/UpperExp
        // TODO: FromStr
      }
    )+
  }
}

tests!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 char bool);
