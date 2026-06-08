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

// expanding `count! { 5 }`
// to `if 5 == 1 { 1 } else { count! (5 - 1) }`
// expanding `count! { 5 - 1 }`
// to `if 5 - 1 == 1 { 1 } else { count! (5 - 1 - 1) }`
// expanding `count! { 5 - 1 - 1 }`
// to `if 5 - 1 - 1 == 1 { 1 } else { count! (5 - 1 - 1 - 1) }`
