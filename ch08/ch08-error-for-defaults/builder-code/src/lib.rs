mod fields;

use crate::fields::{builder_field_definitions, builder_init_values};
use crate::fields::{builder_methods, optional_default_asserts, original_struct_setters};
use quote::{format_ident, quote};
use syn::{Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed};
use syn::{__private::TokenStream2, parse2};

const DEFAULTS_ATTRIBUTE_NAME: &str = "builder_defaults";

pub fn create_builder(item: TokenStream2) -> TokenStream2 {
  let derive_input: DeriveInput = parse2(item).unwrap();
  let name = derive_input.ident;
  let builder = format_ident!("{}Builder", name);
  let is_using_defaults: bool = derive_input
    .attrs
    .iter()
    .any(|attribute| attribute.path().is_ident(DEFAULTS_ATTRIBUTE_NAME));

  let fields = match derive_input.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { ref named, .. }),
      ..
    }) => named,
    _ => unimplemented!("only implemented for structs"),
  };
  let builder_fields = builder_field_definitions(fields);
  let builder_inits = builder_init_values(fields);
  let builder_methods = builder_methods(fields);
  let set_fields = original_struct_setters(fields, is_using_defaults);

  let default_assertions = if is_using_defaults {
    optional_default_asserts(fields)
  } else {
    vec![]
  };

  quote! {
      struct #builder {
          #(#builder_fields,)*
      }

      impl #builder {
          #(#builder_methods)*

          pub fn build(self) -> #name {
              #name {
                  #(#set_fields,)*
              }
          }
      }

      impl #name {
          pub fn builder() -> #builder {
              #builder {
                  #(#builder_inits,)*
              }
          }
      }

      #(#default_assertions)*
  }
}
