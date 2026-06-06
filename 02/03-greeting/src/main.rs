#![allow(unused)]

use crate::greeting::base_greeting_fn;

#[macro_use]
mod greeting;

fn main() {
  let greet = greeting!("Heya", "Sam");
  println!("{}", greet);

  let greet_with_default = greeting!("Sam");
  println!("{}", greet_with_default);
}

mod macro_dev {
  use super::*;
  fn test1() {
    let actual = greeting!("Hans");
    let actual = greeting!("Willkommen", "Hans");
  }
}
