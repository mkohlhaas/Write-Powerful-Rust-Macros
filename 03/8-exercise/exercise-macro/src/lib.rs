use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(UpperCaseName)]
pub fn uppercase(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item);

  // print AST
  eprintln!("{:#?}", ast);

  let name = ast.ident;
  let name_str = name.to_string();
  let uppercase_name = name_str.to_uppercase();

  quote! {
    impl #name {
      // methods
      fn uppercase(&self) {
          println!("{}", stringify!(#name));
          println!("{}", #uppercase_name);
      }

      fn greeting(&self) {
          println!("Hello {}! ", #name_str);
          println!("Hello {}!", stringify!(#name));
          println!("Hello {:?}!", #name);
      }

      // associated functions
      fn testing_testing() {
          println!("One two three.");
      }
    }
  }
  .into()
}
