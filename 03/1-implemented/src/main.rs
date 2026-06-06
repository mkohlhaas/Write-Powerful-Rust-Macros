// #[macro_use]
// extern crate hello_world_implemented_macro;

use hello_world_implemented_macro::Hello;

#[derive(Hello)]
struct Example;

impl Example {
  fn another_function(&self) {
    println!("Something else from Example.");
  }
}

#[derive(Hello)]
enum Pet {
  Cat,
}

fn main() {
  // Example
  let e = Example {};
  e.hello_world();
  e.another_function();

  // Pet
  let p = Pet::Cat;
  p.hello_world();
}
