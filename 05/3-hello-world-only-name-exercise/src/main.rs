#![allow(unused)]

use hello_world_only_name_exercise_macro::hello;

struct Example {
  another_value: String,
}

hello!(Example);

fn main() {
  let ex = Example {
    another_value: "does not disappear".to_string(),
  };

  ex.hello_world();
}

#[cfg(test)]
mod tests {
  use crate::Example;

  #[test]
  fn value_does_not_disappear() {
    let ex = Example {
      another_value: "does not disappear".to_string(),
    };

    assert_eq!(ex.another_value, "does not disappear".to_string());
  }

  #[test]
  fn hello_world_method_available() {
    let ex = Example {
      another_value: "does not disappear".to_string(),
    };

    ex.hello_world();
  }
}
