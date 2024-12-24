#![allow(unused)]

#[derive(Debug)]
struct Car {
  wheels: u8,
  gps: String,
  infotainment: String,
}

fn steal(item: String) {
  println!("I am stealing {item}.");
}

fn main() {
  let car = Car {
    wheels: 4,
    gps: "Garmin".to_string(),
    infotainment: "Android".to_string(),
  };
  println!("My car before the theft: {car:?}.");
  steal(car.gps);
  // "value partially moved" when uncommenting this line:
  // println!("My car after the theft: {car:?}"); // does not compile
}
