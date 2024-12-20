#![allow(unused)]
#![feature(trace_macros)]
#![feature(log_syntax)]

use crate::greeting::base_greeting_fn;
#[macro_use]
mod greeting;

fn main() {
  trace_macros!(true);
  // let _greet = greeting!("Sam", "Heya");
  // let _greet_with_default = greeting!("Sam");
  let greet_with_default_test = greeting!(test "Sam");
  println!("{}", greet_with_default_test);
  // trace_macros!(false);
}

mod macro_dev {
  use super::*;
  fn macro_test1() {
    let actual = greeting!("Sam", "Heya");
    let actual = greeting!("Sam");
    let greet_with_default_test = greeting!(test "Sam");
  }
}
