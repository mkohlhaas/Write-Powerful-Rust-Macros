#![allow(dead_code, unused)]

fn add_one(n: i32) -> i32 {
  n + 1
}

fn stringify(n: i32) -> String {
  n.to_string()
}

fn prefix_with(prefix: &str) -> impl Fn(String) -> String + '_ {
  move |x| format!("{}{}", prefix, x)
}

fn compose_two<FIRST, SECOND, THIRD, F, G>(f: F, g: G) -> impl Fn(FIRST) -> THIRD
where
  F: Fn(FIRST) -> SECOND,
  G: Fn(SECOND) -> THIRD,
{
  move |x| g(f(x))
}

macro_rules! compose {
    ($head:expr) => { $head };
    ($head:expr, $($tail:expr),+) => {
        compose_two($head, compose!($($tail),+))
    }
}

macro_rules! compose_alt {
    ($head:expr) => { $head };
    ($head:expr => $($tail:expr)=>+) => {
        compose_two($head, compose_alt!($($tail)=>+))
    }
}

fn main() {
  let two_composed_function = compose_two(compose_two(add_one, stringify), prefix_with("Result: "));
  println!("{:?}", two_composed_function(5));

  let composed = compose!(add_one, stringify, prefix_with("Result: "));
  println!("{:?}", composed(5));

  let composed = compose_alt!(
      add_one => stringify => prefix_with("Result: ")
  );
  println!("{:?}", composed(5));
}

mod macro_dev {
  use super::*;
  fn test1() {
    let composed = compose!(add_one);
    let composed = compose!(add_one, stringify);
    let composed = compose!(add_one, stringify, prefix_with("Result: "));
    let composed = compose_alt!(add_one);
    let composed = compose_alt!(add_one => stringify);
    let composed = compose_alt!(add_one => stringify => prefix_with("Result: "));
  }
}
