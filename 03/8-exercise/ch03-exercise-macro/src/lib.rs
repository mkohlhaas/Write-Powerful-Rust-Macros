use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(UpperCaseName)]
pub fn uppercase(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item);

  // print AST
  eprintln!("{:#?}", &ast);

  let name = ast.ident;
  let name_str = name.to_string();
  let uppercase_name = name.to_string().to_uppercase();

  quote! {
    impl #name {
      fn uppercase(&self) {
          println!("{}", #uppercase_name);
      }
      fn testing_testing() {
          println!("One two three.");
      }
      fn greeting(&self) {
          println!("Hello {}!", #name_str);
          println!("Hello {}!", stringify!(#name));
      }
    }
  }
  .into()
}
