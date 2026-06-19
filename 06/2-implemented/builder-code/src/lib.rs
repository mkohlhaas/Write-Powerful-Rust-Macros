use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data::Struct, DataStruct, DeriveInput, Field, Fields::Named, FieldsNamed, Ident};

pub fn create_builder(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse2(item).unwrap();
  let name_original_struct = ast.ident;
  let builder_name = format_ident!("{}Builder", name_original_struct);

  let fields = match ast.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { ref named, .. }),
      ..
    }) => named,
    _ => unimplemented!("only implemented for structs"),
  };

  // optionize struct fields
  let builder_fields = fields.iter().map(|field| {
    let field_name = &field.ident;
    let field_type = &field.ty;
    quote! { #field_name: Option<#field_type> }
  });

  // initialize builder fields to None
  let builder_inits = fields.iter().map(|field| {
    let field_name = &field.ident;
    quote! { #field_name: None }
  });

  // create setters for all struct fields in the builder
  let builder_methods = fields.iter().map(|field| {
    let field_name = &field.ident;
    let field_type = &field.ty;
    quote! {
        pub fn #field_name(&mut self, input: #field_type) -> &mut Self {
            self.#field_name = Some(input);
            self
        }
    }
  });

  // copy values from builder to original struct
  let set_fields_in_original_struct = fields.iter().map(|field: &Field| {
    let field_name: &Option<Ident> = &field.ident;
    let field_name_as_string = field_name.as_ref().unwrap().to_string();

    // NOTE:
    // "Any type implementing the ToTokens trait can be interpolated."
    // https://docs.rs/quote/latest/quote/macro.quote.html#interpolation
    // Options can be interpolated as they implement ToTokens and interpolate what's inside `Some`.
    // `None`s are ignored.
    // https://docs.rs/quote/latest/src/quote/to_tokens.rs.html#114-120
    // Same for `str`.
    // https://docs.rs/quote/latest/src/quote/to_tokens.rs.html#122-126

    // NOTE: For now we expect fields to be of type `String`.
    quote! {
        // copy fields
        #field_name: self.#field_name.as_ref()
            .expect(&format!("field {} not set", #field_name_as_string))
            .to_string() // as we can’t just “move” out of the field, we do a to_string to get a copy of the value
    }
  });

  quote! {
      struct #builder_name {
          #(#builder_fields,)*
      }

      impl #builder_name {
          #(#builder_methods)*

          pub fn build(&self) -> #name_original_struct {
              #name_original_struct {
                  #(#set_fields_in_original_struct,)*
              }
          }
      }

      impl #name_original_struct {
          pub fn builder() -> #builder_name {
              #builder_name {
                  #(#builder_inits,)*
              }
          }
      }
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
    assert_eq!(
      actual
        .to_string()
        .split_ascii_whitespace()
        .skip(1) // skip "struct"
        .next()
        .unwrap(),
      "StructWithNoFieldsBuilder"
    );
  }

  #[test]
  fn builder_struct_with_expected_methods_should_be_present_in_output() {
    let input: TokenStream = quote! {
        struct StructWithNoFields {}
    };

    let expected: TokenStream = quote! {
        struct StructWithNoFieldsBuilder {}

        impl StructWithNoFieldsBuilder {
            pub fn build(&self) -> StructWithNoFields {
                StructWithNoFields {}
            }
        }

        impl StructWithNoFields {
            pub fn builder() -> StructWithNoFieldsBuilder {
                StructWithNoFieldsBuilder {}
            }
        }
    };

    let actual: TokenStream = create_builder(input);

    assert_eq!(actual.to_string(), expected.to_string());
  }
}
