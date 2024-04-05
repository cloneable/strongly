use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn typed(_attr: TokenStream, input: TokenStream) -> TokenStream {
  let item = input.clone();
  let _item = parse_macro_input!(item as ItemStruct);

  input
}
