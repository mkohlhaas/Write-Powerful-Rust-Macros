use std::marker::PhantomData;

trait Light {}

struct Green {}
struct Red {}

// used to encode state
impl Light for Green {}
impl Light for Red {}

// Light defines the kind of TrafficLight: TrafficLight<Red> or TrafficLight<Green>
// Because Rust does not allow unused generic properties, we have to add
// PhantomData<T> (from the standard library) to TrafficLight.
struct TrafficLight<T: Light> {
  marker: PhantomData<T>,
}

impl TrafficLight<Green> {
  fn turn_red(&self) -> TrafficLight<Red> {
    TrafficLight {
      marker: Default::default(),
    }
  }
}

impl TrafficLight<Red> {
  fn turn_green(&self) -> TrafficLight<Green> {
    TrafficLight {
      marker: Default::default(),
    }
  }
}

fn main() {
  // Compiler can figure out the kind of light:
  // TrafficLight<Red> or TrafficLight<Green>
  let light = TrafficLight {
    marker: Default::default(),
  };

  // Switch the following two lines and see the type of light changing:
  // light.turn_red().turn_green();
  light.turn_green().turn_red();

  // impossible
  // light.turn_red().turn_red();
  // light.turn_green().turn_green();
}
