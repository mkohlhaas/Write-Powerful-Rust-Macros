use hello_world_exercise_macro::UpperCaseName;

#[derive(Debug, UpperCaseName)]
struct Example;

fn main() {
  {
    let e = Example;

    e.greeting();
    println!();
    e.uppercase();
  }

  println!();

  {
    Example::testing_testing();
  }
}
