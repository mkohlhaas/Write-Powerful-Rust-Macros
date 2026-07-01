use crate::input::ConfigInput;
use crate::output::generate_config_struct;
use proc_macro::TokenStream;
use proc_macro2::Span;
use std::collections::HashMap;
use std::fs;
use syn::parse_macro_input;

mod input;
mod output;

const CONFIG_FILE: &str = "./configuration/config.yaml";

// NOTE: Interaction with the local file system happens at compile-time!!!
//       Similar to type providers in F# !!!
fn find_yaml_values(input: ConfigInput) -> Result<HashMap<String, String>, syn::Error> {
  println!("Still in compilation mode (compile-time).");
  let file_name = input.path.unwrap_or(CONFIG_FILE.to_string());

  let file = fs::File::open(&file_name).map_err(|err| {
    syn::Error::new(
      Span::call_site(),
      format!("could not read config with path {}: {}", file_name, err),
    )
  })?;
  println!("{:?}", file);

  serde_yaml::from_reader(file).map_err(|err| syn::Error::new(Span::call_site(), err.to_string()))
}

#[proc_macro]
pub fn config(item: TokenStream) -> TokenStream {
  let input: ConfigInput = parse_macro_input!(item);

  match find_yaml_values(input) {
    Ok(values) => generate_config_struct(values).into(),
    Err(e) => e.into_compile_error().into(),
  }
}
