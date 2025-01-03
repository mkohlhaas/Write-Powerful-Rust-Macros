use std::collections::HashMap;

use config_macro::config_struct;

// #[config_struct(exclude = "from")]
#[config_struct()]
#[derive(Debug)]
struct ConfigStruct {}

fn main() {
  let config = ConfigStruct::new();
  println!("{config:#?}");
  // let user_admin_map = HashMap::from(config);
  let user_admin_map: HashMap<_, _> = config.into();
  println!("{:#?}", user_admin_map);
}

#[cfg(test)]
mod tests {
  use config_macro::{config, config_struct};
  use std::collections::HashMap;

  #[test]
  fn should_generate_config_struct_with_expected_values() {
    config!();

    let cfg = Config::new();
    let user = cfg.0.get("user").unwrap();

    assert_eq!(user, "admin");
  }

  #[test]
  fn should_generate_config_for_existing_struct_with_from_method() {
    #[config_struct(path = "./config-macro-usage/tests/configuration/config.yaml")]
    struct MyConfigStruct {}

    let cfg = MyConfigStruct::new();
    let as_map: HashMap<String, String> = cfg.into();

    assert_eq!(as_map.get("user").unwrap(), "test");
  }
}
