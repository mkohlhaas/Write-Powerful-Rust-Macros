#![allow(unused)]
#![feature(trace_macros)]

macro_rules! hello_world {
  ($something:ident) => {
    impl $something {
      fn hello_world(&self) {
        println!("Hello world")
      }
    }
  };
}

trace_macros!(true);

struct Example;

hello_world!(Example);

fn main() {
  let e = Example;
  e.hello_world(); // prints "Hello world"
}
