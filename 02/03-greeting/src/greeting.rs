pub fn base_greeting_fn(greeting: &str, name: &str) -> String {
  format!("{} {}!", greeting, name)
}

macro_rules! greeting {
  ($name:literal) => {
    base_greeting_fn("Hello", $name)
  };
  ($greeting:literal, $name:literal) => {
    base_greeting_fn($greeting, $name)
  };
}

// pub(crate) use greeting;
