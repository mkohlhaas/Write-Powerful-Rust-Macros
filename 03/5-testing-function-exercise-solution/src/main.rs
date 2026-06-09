#[macro_use]
extern crate hello_world_testing_function_macro;

#[derive(Debug, Hello)]
struct Example;

fn main() {
  {
    let e = Example {};
    println!("{:?}", e);
    e.hello_world();
  }

  println!();

  {
    Example::testing_testing();
  }
}
