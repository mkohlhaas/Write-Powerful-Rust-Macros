use crate::example::{AlsoWorksExample, Example};

mod example;

fn main() {
  let _ = AlsoWorksExample {
    first: "".to_string(),
    second: 0,
  };

  let ex = Example::new();

  println!("{}", ex.first);
  println!("{}", ex.second);

  // println!("{}", ex.third); // won't work
}
