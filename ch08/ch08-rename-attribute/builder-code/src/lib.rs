mod fields;

use crate::fields::{builder_field_definitions, builder_init_values};
use crate::fields::{builder_methods, original_struct_setters};
use quote::{format_ident, quote};
use syn::{parse2, DeriveInput, __private::TokenStream2};
use syn::{Data::Struct, DataStruct, Fields::Named, FieldsNamed};

pub fn create_builder(item: TokenStream2) -> TokenStream2 {
  let derive_input: DeriveInput = parse2(item).unwrap();
  // eprintln!("{:#?}", derive_input.data);
  let struct_name = derive_input.ident;
  // eprintln!("{:#?}", struct_name);
  let builder = format_ident!("{}Builder", struct_name);

  let fields = match derive_input.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { ref named, .. }),
      ..
    }) => named,
    _ => unimplemented!("only implemented for structs"),
  };
  // eprintln!("{:#?}", fields);
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

          pub fn build(self) -> #struct_name {
              #struct_name {
                  #(#set_fields,)*
              }
          }
      }

      impl #struct_name {
          pub fn builder() -> #builder {
              #builder {
                  #(#builder_inits,)*
              }
          }
      }
  }
}
