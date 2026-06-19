use syn_errors_exercise::private;

private!(
  struct Example {
    pub string_value: String,
    pub number_value: i32,
  }
);

// rust-analyzer: only works for structs with named fields [macro-error]
// private!(
//   struct Example1(String, i32);
// );

// rustc: does not work for enums
// private!(
//   enum Example {
//     First,
//   }
// );
