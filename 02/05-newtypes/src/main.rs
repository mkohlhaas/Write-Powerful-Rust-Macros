struct FirstNameExample {
  value: String,
}

impl FirstNameExample {
  pub fn new(name: &str) -> Result<FirstNameExample, String> {
    if name.len() < 2 {
      Err("Name should be at least two characters".into())
    } else {
      Ok(FirstNameExample {
        value: name.to_string(),
      })
    }
  }

  pub fn get_value(&self) -> &String {
    &self.value
  }
}

#[derive(Debug)]
struct FirstName {
  value: String,
}

#[derive(Debug)]
struct LastName {
  value: String,
}

#[derive(Debug)]
struct Age {
  value: u8,
}

#[derive(Debug)]
struct Pay {
  value: i32,
}

macro_rules! generate_newtypes_methods {
  ($struct_name:ident) => {
    impl $struct_name {
      pub fn new(name: &str) -> Result<$struct_name, String> {
        if name.len() < 2 {
          Err("Name should be at least two characters".into())
        } else {
          Ok($struct_name { value: name.into() })
        }
      }
    }
  };
}

macro_rules! generate_get_value_string {
  ($struct_type:ident) => {
    generate_get_value_string!($struct_type, String);
  };
  ($struct_type:ident,$return_type:ty) => {
    impl $struct_type {
      pub fn get_value(&self) -> &$return_type {
        &self.value
      }
    }
  };
}

macro_rules! generate_from_trait {
  ($struct_name:ident, $from_type:ty) => {
    impl From<$from_type> for $struct_name {
      fn from(value: $from_type) -> Self {
        Self::new(value).unwrap()
      }
    }
  };
}
macro_rules! generate_from_primitive_trait {
  ($struct_name:ident, $from_type:ty) => {
    impl From<$from_type> for $struct_name {
      fn from(value: $from_type) -> Self {
        Self { value }
      }
    }
  };
}

generate_get_value_string!(FirstName);
generate_get_value_string!(LastName);
generate_get_value_string!(Age, u8);
generate_get_value_string!(Pay, i32);

generate_newtypes_methods!(FirstName);
generate_newtypes_methods!(LastName);

generate_from_trait!(FirstName, &str);
generate_from_trait!(LastName, &str);
generate_from_primitive_trait!(Age, u8);
generate_from_primitive_trait!(Pay, i32);

fn calculate_raise(
  first_name: FirstName,
  _last_name: LastName,
  _age: Age,
  current_pay: Pay,
) -> Pay {
  if first_name.get_value() == "Sam" {
    Pay {
      value: current_pay.get_value() + 1000,
    }
  } else {
    current_pay
  }
}

fn main() {
  let first_raise = calculate_raise("Sam".into(), "Smith".into(), 20.into(), 1000.into());
  println!("{:?}", first_raise);

  let first_raise = calculate_raise(
    FirstName::from("Sam"),
    LastName::from("Smith"),
    Age::from(20),
    Pay::from(1000),
  );
  println!("{:?}", first_raise);
}

#[cfg(test)]
mod tests {
  use crate::FirstNameExample;

  #[test]
  fn should_create_first_name_example() {
    let actual = FirstNameExample::new("Sam").unwrap();
    assert_eq!(actual.get_value(), "Sam");
  }

  #[test]
  fn should_fail_to_create_first_name_example_that_is_too_short() {
    let actual = FirstNameExample::new("S");
    assert!(actual.is_err());
  }
}
