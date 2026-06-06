use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Hello)]
pub fn hello(_item: TokenStream) -> TokenStream {
  let add_hello_world = quote! {};
  eprintln!("Token Stream: {:#?}", &add_hello_world);
  add_hello_world.into()
}
