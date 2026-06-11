use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, parse_macro_input};

#[proc_macro]
pub fn hello(item: TokenStream) -> TokenStream {
  let ast: Ident = parse_macro_input!(item);
  // eprintln!("{:#?}", &ast);

  quote!(
      impl #ast {
          fn hello_world(&self) {
              println!("Hello world!")
          }
      }
  )
  .into()
}
