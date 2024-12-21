fn main() {
  println!("Hello, world!");
}

macro_rules! count {
  ($val: expr) => {
    if $val == 1 {
      1
    } else {
      count!($val - 1)
    }
  };
}

const CONST: i32 = count!(5);
