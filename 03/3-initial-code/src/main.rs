// #[macro_use]
// extern crate hello_world_macro;

use hello_world_macro::Hello;

#[derive(Hello, Debug)]
struct Example;

fn main() {
  let example = Example;
  println!("{:?}", example);
}
