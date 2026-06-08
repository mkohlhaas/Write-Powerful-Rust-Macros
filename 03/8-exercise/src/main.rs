use hello_world_exercise_macro::UpperCaseName;

#[derive(UpperCaseName)]
struct Example;

fn main() {
  let e = Example;

  e.uppercase();
  e.greeting();

  Example::testing_testing();
}
