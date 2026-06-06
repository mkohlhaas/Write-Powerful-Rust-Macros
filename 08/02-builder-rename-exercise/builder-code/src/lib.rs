mod fields;

use crate::fields::{
  builder_field_definitions, builder_init_values, builder_methods, original_struct_setters,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Data::Struct;
use syn::DataStruct;
use syn::Fields::Named;
use syn::FieldsNamed;
use syn::{parse2, DeriveInput};

pub fn create_builder(item: TokenStream) -> TokenStream {
  let derive_input: DeriveInput = parse2(item).unwrap();
  // eprintln!("{:#?}", derive_input);
  let name = derive_input.ident;
  let builder = format_ident!("{}Builder", name);

  let fields = match derive_input.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { named, .. }),
      ..
    }) => named,
    _ => unimplemented!("only implemented for structs"),
  };
  let builder_fields = builder_field_definitions(&fields);
  let builder_inits = builder_init_values(&fields);
  let builder_methods = builder_methods(&fields);
  let set_fields = original_struct_setters(&fields);

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
