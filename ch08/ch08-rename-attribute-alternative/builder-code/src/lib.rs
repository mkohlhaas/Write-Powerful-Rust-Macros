mod fields;

use crate::fields::{builder_field_definitions, builder_init_values};
use crate::fields::{builder_methods, original_struct_setters};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse2, Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed};

pub fn create_builder(item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse2(item).unwrap();
  let name = ast.ident;
  let builder = format_ident!("{}Builder", name);

  let fields = match ast.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { ref named, .. }),
      ..
    }) => named,
    _ => unimplemented!("only implemented for structs"),
  };
  let builder_fields = builder_field_definitions(fields);
  let builder_inits = builder_init_values(fields);
  let builder_methods = builder_methods(fields);
  let set_fields = original_struct_setters(fields);

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
  }
}
