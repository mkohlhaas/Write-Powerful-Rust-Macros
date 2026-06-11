use private_macro_with_private_fields::private;

private!(
  struct Example {
    pub string: String,
    pub number: i32,
  }
);
