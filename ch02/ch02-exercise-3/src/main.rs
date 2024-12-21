#![allow(unused)]

use curry_macro::curry;

fn main() {
  // without any type annotations
  let add = curry!(|a, b| a + b);
  assert_eq!(add(1)(2), 3);

  // with type annotations
  let add = curry!(|a: i32, b: i32| -> i32 { a + b });

  // You can generate intermediate functions that are partially applied.
  let add1_to = add(1);
  let sum = add1_to(2);
  // assert_eq!(sum, 3);

  // You can also can apply all arguments at once.
  let sum = add(1)(2);
  // assert_eq!(sum, 3);
}
