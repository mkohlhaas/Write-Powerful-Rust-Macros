#[macro_use]
extern crate venial_macro;

#[derive(Hello)]
struct Example;

#[derive(Hello)]
enum Pet {
  Cat,
}

fn main() {
  // Example
  let e = Example;
  e.hello_world();

  // Pet
  let p = Pet::Cat;
  p.hello_world();
}
