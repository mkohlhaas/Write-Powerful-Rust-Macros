#![allow(unused, clippy::vec_init_then_push)]

macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($($x:expr),+ $(,)?) => (
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )+
            v
        }
    );
}

fn main() {}

#[cfg(test)]
mod tests {

  #[test]
  fn should_create_vec_with_given_elements() {
    let actual = my_vec!(1, 2, 3);

    assert_eq!(actual.len(), 3);
    assert_eq!(actual[0], 1);
    assert_eq!(actual[1], 2);
    assert_eq!(actual[2], 3);
  }

  #[test]
  fn should_create_vec_with_trailing_comma() {
    let actual = my_vec!(1, 2, 3,);

    assert_eq!(actual.len(), 3);
    assert_eq!(actual[0], 1);
    assert_eq!(actual[1], 2);
    assert_eq!(actual[2], 3);
  }
}

mod macro_dev {
  fn tst() {
    let actual = my_vec!(1, 2, 3);
    let actual = my_vec!(1, 2, 3,);
    let actual = my_vec!(1, 2, 3, 4, 5, 6, 7, 8, 9,);
  }
}
