use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item);

  eprintln!("{:#?}", ast);

  let name: Ident = ast.ident;
  let name_str = name.to_string();

  let add_hello_world: TokenStream2 = quote! {
      impl #name {
          fn hello_world(&self) {
              println!("Hello world from {}.", #name_str)
          }
      }
  };

  add_hello_world.into()
}
