#![allow(clippy::vec_init_then_push)]

macro_rules! my_vec {
  // empty vec
  () => {
    Vec::new()
  };

  // empty vec
  (make an empty vec) => {
    Vec::new()
  };

  // single-item vec
  ($x:literal) => {{
    let mut v = Vec::new();
    v.push($x);
    v
  }};

  // multi-item vec
  ($($x:literal),+$(,)?) => {{
    let mut v = Vec::new();
    $(
        v.push($x);
    )+
    v
  }};
}

fn main() {
  let empty1: Vec<i32> = my_vec![];
  let empty2: Vec<i32> = my_vec!(make an empty vec);
  let t1 = my_vec![1];
  let t2 = my_vec![1,];
  let t3 = my_vec![1, 2, 3];
  let t4 = my_vec![1, 2, 3,];

  println!("{:?}", empty1);
  println!("{:?}", empty2);
  println!("{:?}", t1);
  println!("{:?}", t2);
  println!("{:?}", t3);
  println!("{:?}", t4);
}

#[cfg(test)]
mod tests {
  #[test]
  fn should_create_empty_vec() {
    let actual: Vec<i32> = my_vec!();
    assert_eq!(actual.len(), 0);
  }
  #[test]
  fn should_create_empty_vec_alt() {
    let actual: Vec<i32> = my_vec!(make an empty vec);
    assert_eq!(actual.len(), 0);
  }

  #[test]
  fn should_create_vec_with_one_element() {
    let actual = my_vec!(1);
    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], 1);
  }

  #[test]
  fn should_create_vec_with_one_element_trailing_comma() {
    let actual = my_vec!(1,);
    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0], 1);
  }

  #[test]
  fn should_create_vec_with_given_elements() {
    let actual = my_vec!(1, 2, 3);
    assert_eq!(actual.len(), 3);
    assert_eq!(actual[0], 1);
    assert_eq!(actual[1], 2);
    assert_eq!(actual[2], 3);
  }
}
