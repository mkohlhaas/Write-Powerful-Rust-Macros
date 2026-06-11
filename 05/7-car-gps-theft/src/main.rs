// Explaining partial moves 2/2

// Solutions:
// 1. use a reference
// 2. clone
// 3. take(…), replace(…), …

#![allow(unused)]

#[derive(Debug)]
struct Car {
  wheels: u8,
  gps: Option<String>,
  infotainment: String,
}

fn steal(item: String) {
  println!("I am stealing {item}.");
}

fn main() {
  let mut car = Car {
    wheels: 4,
    gps: Some("Garmin".to_string()),
    infotainment: "Android".to_string(),
  };
  println!("My car before the theft: {car:?}.");

  // NOTE: use take or replace to avoid partial moves
  steal(car.gps.take().unwrap());

  // works, though the gps is now missing (None)
  println!("My car after theft: {car:?}.");
}
