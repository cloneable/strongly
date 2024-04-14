macro_rules! types {
  (  $($inner:ident)+ ) => {
    $(
      paste::paste! {
        #[::strongly::typed(pub $inner)]
        pub struct [<Strong $inner>];
      }
    )+
  };
}

types!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 char bool);
