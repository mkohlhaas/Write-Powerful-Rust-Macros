trait Hello {
  fn hello(&self);
}

// impl<T: Sized> Hello for T {
impl<T: Copy> Hello for T {
  fn hello(&self) {
    println!("Hello world");
  }
}

fn main() {
  2.hello();
  true.hello();
  'c'.hello();
  "".hello();
  // "".to_string().hello();
}
