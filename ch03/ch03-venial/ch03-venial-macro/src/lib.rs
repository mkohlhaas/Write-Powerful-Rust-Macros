use proc_macro::TokenStream;
use quote::quote;
use venial::{parse_declaration, Declaration, Enum, Struct};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
  // Convert from proc_macro::TokenStream into proc_macro2::TokenStream
  let declaration = parse_declaration(item.into()).unwrap();

  // print Declaration AST
  eprintln!("{:#?}", declaration);

  let name /* : proc_macro2::Ident */ = match declaration {
    Declaration::Struct(Struct { name, .. }) => name,
    Declaration::Enum(Enum { name, .. }) => name,
    _ => panic!("only implemented for struct and enum"),
  };

  let add_hello_world = quote! {
      impl #name {
          fn hello_world(&self) {
              println!("Hello world.")
          }
      }
  };

  add_hello_world.into()
}
