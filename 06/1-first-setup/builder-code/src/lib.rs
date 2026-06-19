use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

pub fn create_builder(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse2(item).unwrap();
  let name = ast.ident;
  let builder = format_ident!("{}Builder", name);

  quote! {
      struct #builder {}
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn builder_struct_name_should_be_present_in_output() {
    let input = quote! {
        struct StructWithNoFields {}
    };
    let actual = create_builder(input);
    assert!(actual.to_string().contains("StructWithNoFieldsBuilder"));
  }

  #[test]
  fn builder_struct_with_expected_methods_should_be_present_in_output() {
    let input = quote! {
        struct StructWithNoFields {}
    };
    let actual = create_builder(input);

    let expected = quote! {
        struct StructWithNoFieldsBuilder {}
    };
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn assert_with_parsing() {
    let input = quote! {
        struct StructWithNoFields {}
    };
    let actual = create_builder(input);

    // `DeriveInput` is a data structure from the syn crate that represents the Abstract Syntax Tree
    // (AST) of a struct, enum, or union when writing a custom derive procedural macro.
    // We know that `actual` represents a struct. So we can parse it as such.
    let actual_derived: DeriveInput = syn::parse2(actual).unwrap();
    let name = actual_derived.ident;
    assert_eq!(name.to_string(), "StructWithNoFieldsBuilder");
  }
}
