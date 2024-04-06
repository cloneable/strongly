use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ItemStruct, Result};

#[proc_macro_attribute]
pub fn typed(
  params: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  match typed_main(params.into(), input.into()) {
    Ok(output) => output.into(),
    Err(err) => err.into_compile_error().into(),
  }
}

fn typed_main(params: TokenStream, input: TokenStream) -> Result<TokenStream> {
  if !params.is_empty() {
    return Err(Error::new(
      Span::call_site(),
      String::from("This macro does not (yet) accept parameters"),
    ));
  }

  let _item = syn::parse2::<ItemStruct>(input.clone())?;

  let mut output = TokenStream::new();

  // Emit unchanged struct with the nine default derives.
  // Deriving PartialEq,Eq also gives us StructuralPartialEq.
  output.extend(quote! {
    #[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
  });
  output.extend(input);

  Ok(output)
}
