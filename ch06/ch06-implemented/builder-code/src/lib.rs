use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed};

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
  let builder_fields = fields.iter().map(|f| {
    let field_name = &f.ident;
    let field_type = &f.ty;
    quote! { #field_name: Option<#field_type> }
  });

  // initialize builder fields to None
  let builder_inits = fields.iter().map(|f| {
    let field_name = &f.ident;
    quote! { #field_name: None }
  });

  // create setters for all struct fields in the builder
  let builder_methods = fields.iter().map(|f| {
    let field_name = &f.ident;
    let field_type = &f.ty;
    quote! {
        pub fn #field_name(&mut self, input: #field_type) -> &mut Self {
            self.#field_name = Some(input);
            self
        }
    }
  });

  // copy values from builder to original struct
  let set_fields_in_original_struct = fields.iter().map(|f| {
    let field_name = &f.ident;
    let field_name_as_string = field_name.as_ref().unwrap().to_string();

    quote! {
        // copy fields
        #field_name: self.#field_name.as_ref()
            .expect(&format!("field {} not set", #field_name_as_string))
            .to_string()
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
  }

  #[test]
  fn builder_struct_with_expected_methods_should_be_present_in_output() {
    let input = quote! {
        struct StructWithNoFields {}
    };
    let expected = quote! {
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

    let actual = create_builder(input);

    assert_eq!(actual.to_string(), expected.to_string());
  }
}
