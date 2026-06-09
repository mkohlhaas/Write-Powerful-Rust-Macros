struct Dropper {}

impl Drop for Dropper {
  fn drop(&mut self) {
    println!("Dropping!") // will be called despite panic
  }
}

#[allow(unreachable_code)]
fn some_fun() {
  let _d = Dropper {};
  panic!("panic");
  core::mem::forget(_d);
}

fn main() {
  some_fun(); // "Dropping"
}
