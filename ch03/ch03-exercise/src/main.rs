use hello_world_exercise_macro::UpperCaseName;

#[derive(UpperCaseName)]
struct Example;

fn main() {
  let e = Example;
  e.uppercase();
  Example::testing_testing();
  e.greeting();
}
