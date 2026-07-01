use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;

fn generate_inserts(yaml_values: HashMap<String, String>) -> Vec<TokenStream> {
  yaml_values
    .iter()
    .map(|kv| {
      let key = kv.0;
      let value = kv.1;
      quote!(map.insert(#key.to_string(), #value.to_string());)
    })
    .collect()
}

pub fn generate_config_struct(yaml_values: HashMap<String, String>) -> TokenStream {
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
