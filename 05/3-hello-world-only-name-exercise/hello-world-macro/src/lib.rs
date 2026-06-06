use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn hello(item: TokenStream) -> TokenStream {
  let ast: Ident = parse_macro_input!(item);
  // eprintln!("{:#?}", &ast);

  quote!(
      impl #ast {
          fn hello_world(&self) {
              println!("Hello world")
          }
      }
  )
  .into()
}
