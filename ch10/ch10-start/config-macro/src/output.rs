use quote::quote;
use std::collections::HashMap;
use syn::__private::TokenStream2;

fn generate_inserts(yaml_values: HashMap<String, String>) -> Vec<TokenStream2> {
  yaml_values
    .iter()
    .map(|v| {
      let key = v.0;
      let value = v.1;
      quote!(map.insert(#key.to_string(), #value.to_string());)
    })
    .collect()
}

pub fn generate_config_struct(yaml_values: HashMap<String, String>) -> TokenStream2 {
  let inserts = generate_inserts(yaml_values);

  quote! {
      pub struct Config(pub std::collections::HashMap<String,String>);

      impl Config {
          pub fn new() -> Self {
              let mut map = std::collections::HashMap::new();
              #(#inserts)*
              Config(map)
          }
      }
  }
}
