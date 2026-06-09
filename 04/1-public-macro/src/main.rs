use make_public_macro::public;

#[derive(Debug)]
#[public]
struct Example {
  first: String,
  pub second: u32,
}

fn main() {
  let example = Example {
    first: "first".into(),
    second: 5,
  };

  println!("{:?}", example);
}
