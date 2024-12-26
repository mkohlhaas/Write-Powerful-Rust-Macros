#![allow(unused)]

mod other_file;
use other_file::Example;

fn main() {
  let e = Example::new();
  e.get_string_value();
  e.get_number_value();
}
