use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item);
  eprintln!("{:#?}", &ast);
  let name = ast.ident;
  let name_str = name.to_string();

  let add_hello_world = quote! {
      impl #name {
          fn hello_world(&self) {
              println!("Hello world from {}.", #name_str)
          }
      }
  };

  add_hello_world.into()
}
