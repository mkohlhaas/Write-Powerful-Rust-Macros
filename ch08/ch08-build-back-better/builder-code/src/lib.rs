mod fields;
mod util;

use crate::fields::{builder_definition, builder_impl_for_struct};
use crate::fields::{builder_methods, marker_trait_and_structs};
use quote::quote;
use syn::{parse2, DeriveInput, __private::TokenStream2};
use syn::{Data::Struct, DataStruct, Fields::Named, FieldsNamed};

pub fn create_builder(item: TokenStream2) -> TokenStream2 {
  let derive_input: DeriveInput = parse2(item).unwrap();
  let name = derive_input.ident;

  let fields = match derive_input.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { named, .. }),
      ..
    }) => named,
    _ => unimplemented!("only implemented for structs"),
  };
  let marker_and_structs = marker_trait_and_structs(&name, &fields);
  let builder = builder_definition(&name, &fields);
  let builder_method_for_struct = builder_impl_for_struct(&name, &fields);
  let builder_methods = builder_methods(&name, &fields);

  quote! {
      #marker_and_structs
      #builder
      #builder_method_for_struct
      #builder_methods
  }
}
