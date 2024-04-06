use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn typed(params: TokenStream, input: TokenStream) -> TokenStream {
  if !params.is_empty() {
    return quote! {
      compile_error!("This macro does not (yet) accept parameters");
    }
    .into();
  }
  let item = input.clone();
  let _item = parse_macro_input!(item as ItemStruct);

  input
}
