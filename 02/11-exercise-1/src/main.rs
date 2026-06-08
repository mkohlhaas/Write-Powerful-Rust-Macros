#![feature(trace_macros)]
#![recursion_limit = "1024"]

fn main() {
  println!("Hello, world!");
}

macro_rules! count {
  ($val: expr) => {
    if $val == 1 { 1 } else { count!($val - 1) }
  };
}

// trace_macros!(true);

const CONST: i32 = count!(5);
