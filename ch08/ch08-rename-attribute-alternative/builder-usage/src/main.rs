fn main() {}

#[cfg(test)]
mod tests {
  use builder_macro::Builder;

  #[test]
  fn should_generate_builder_for_struct_with_one_renamed_property() {
    #[derive(Builder)]
    struct Gleipnir {
      #[rename = "tops_of"]
      roots_of: String,
    }
    let gleipnir = Gleipnir::builder().tops_of("mountains".to_string()).build();
    assert_eq!(gleipnir.roots_of, "mountains".to_string());
  }
}
