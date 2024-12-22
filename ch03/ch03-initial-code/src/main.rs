// #[macro_use]
// extern crate hello_world_macro;

use hello_world_macro::Hello;

#[allow(dead_code)]
#[derive(Hello)]
struct Example;

fn main() {
  let _ = Example;
}
