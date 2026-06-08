#![allow(dead_code, unused)]
#![feature(trace_macros)]

fn add_one(n: i32) -> i32 {
  n + 1
}

fn stringify(n: i32) -> String {
  n.to_string()
}

fn prefix_with(prefix: &str) -> impl Fn(String) -> String + '_ {
  move |x| format!("{}{}", prefix, x)
}

fn compose_two<F, G, H, FN, GN>(f: FN, g: GN) -> impl Fn(F) -> H
where
  FN: Fn(F) -> G,
  GN: Fn(G) -> H,
{
  move |x| g(f(x))
}

macro_rules! compose {
    ($fn:expr) => { $fn };
    ($fn:expr, $($fns:expr),+) => {
        compose_two($fn, compose!($($fns),+))
    }
}

macro_rules! compose_alt {
    ($fn:expr) => { $fn };
    ($fn:expr => $($fns:expr)=>+) => {
        compose_two($fn, compose_alt!($($fns)=>+))
    }
}

fn main() {
  let two_composed_function = compose_two(compose_two(add_one, stringify), prefix_with("Result: "));
  println!("{:?}", two_composed_function(5));

  trace_macros!(true);
  let composed = compose!(add_one, stringify, prefix_with("Result: "));
  trace_macros!(false);
  println!("{:?}", composed(5));

  trace_macros!(true);
  let composed = compose_alt!(
      add_one => stringify => prefix_with("Result: ")
  );
  trace_macros!(false);
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
