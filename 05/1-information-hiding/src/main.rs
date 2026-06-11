use private_macro::private;

private!(
  struct Example {
    string: String,
    number: i32,
  }
);

fn main() {
  let ex = Example {
    string: "I'm a String.".to_string(),
    number: 42,
  };

  println!("String value: {:?}", ex.get_string());
  println!("Number value: {:?}", ex.get_number());
}

#[cfg(test)]
mod tests {
  use crate::Example;

  #[test]
  fn generates_necessary_methods() {
    let ex = Example {
      string: "String".to_string(),
      number: 42,
    };

    assert_eq!(ex.get_string(), &"String");
    assert_eq!(ex.get_number(), &42);
  }
}
