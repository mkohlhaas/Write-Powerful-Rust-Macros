trait Hello1 {
  fn hello1(&self);
}

trait Hello2 {
  fn hello2(&self);
}

impl<T: Sized> Hello1 for T {
  fn hello1(&self) {
    println!("Hello world!");
  }
}

impl<T: Copy> Hello2 for T {
  fn hello2(&self) {
    println!("Hello world!");
  }
}

fn main() {
  {
    2.hello1();
    true.hello1();
    'c'.hello1();
    "".hello1();
    "".to_string().hello1();
  }

  println!();

  {
    2.hello2();
    true.hello2();
    'c'.hello2();
    "".hello2();
    // "".to_string().hello2(); // String is not Copy
  }
}
