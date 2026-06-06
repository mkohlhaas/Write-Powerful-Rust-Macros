use make_public_macro_enum_and_unnamed_struct::public;

#[derive(Debug)]
#[public]
struct Example {
  first: String,
  pub second: u32,
}

#[public]
#[derive(Debug)]
struct UnnamedExample(String, f64);

#[derive(Debug)]
#[public]
enum AnEnumExample {
  First,
  Second,
}

#[public]
#[derive(Debug)]
struct EmptyStruct {}

fn main() {
  let e = Example {
    first: "first".to_string(),
    second: 5,
  };
  println!("{:?}", e);
  let u = UnnamedExample("first".to_string(), 5.2);
  println!("{:?}", u);
  let a = AnEnumExample::First;
  println!("{:?}", a);
  let empty = EmptyStruct {};
  println!("{:?}", empty);
}
