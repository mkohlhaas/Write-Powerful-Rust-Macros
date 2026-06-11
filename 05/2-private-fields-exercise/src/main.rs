use crate::other_file::Example;

mod other_file;

fn main() {
  let ex = Example::new();

  ex.get_string();
  ex.get_number();
}

#[cfg(test)]
mod tests {
  use crate::Example;

  #[test]
  fn generates_necessary_methods() {
    let ex = Example::new();

    assert_eq!(ex.get_string(), &"String");
    assert_eq!(ex.get_number(), &42);
  }
}
