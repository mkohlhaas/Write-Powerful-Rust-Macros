use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(item as DeriveInput);
  let name = ast.ident;

  let add_hello_world = quote! {
      impl #name {
          // methods
          fn hello_world(&self) {
              println!("Hello world.")
          }

          // associated functions
          fn testing_testing() {
              println!("One two three.")
          }
      }
  };

  add_hello_world.into()
}
