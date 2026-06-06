use proc_macro::TokenStream;
use quote::quote;
use std::iter::Map;
use syn::punctuated::{Iter, Punctuated};
use syn::token::Comma;
use syn::Data::{Enum, Struct};
use syn::DataEnum;
use syn::Fields::Unnamed;
use syn::{parse_macro_input, DeriveInput, Field, FieldsNamed, FieldsUnnamed, Ident, Variant};
use syn::{DataStruct, Fields::Named};

type TokenStream2 = proc_macro2::TokenStream;
type MapTokenStream2<'a> = Map<Iter<'a, Field>, fn(&Field) -> TokenStream2>;
type Pfc = Punctuated<Field, Comma>;
type Pvc = Punctuated<Variant, Comma>;

fn named_fields_public(fields: &Pfc) -> MapTokenStream2 {
  fields.iter().map(|f| {
    let name = &f.ident;
    let ty = &f.ty;
    quote! { pub #name: #ty }
  })
}

fn unnamed_fields_public(fields: &Pfc) -> MapTokenStream2 {
  fields.iter().map(|f| {
    let ty = &f.ty;
    quote! { pub #ty }
  })
}

fn generate_named_output(struct_name: Ident, builder_fields: MapTokenStream2) -> TokenStream2 {
  quote!(
      pub struct #struct_name {
          #(#builder_fields,)*
      }
  )
}

fn generate_unnamed_output(struct_name: Ident, builder_fields: MapTokenStream2) -> TokenStream2 {
  quote!(
      pub struct #struct_name(
          #(#builder_fields,)*
      );
  )
}

fn generate_enum_output(enum_name: Ident, variants: &Pvc) -> TokenStream2 {
  let as_iter = variants.into_iter();

  quote!(
      pub enum #enum_name {
          #(#as_iter,)*
      }
  )
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(item as DeriveInput);
  let name = ast.ident;
  let attributes = ast.attrs;
  // eprintln!("{:#?}", &attributes);

  let basic_output = match ast.data {
    // named fields
    Struct(DataStruct {
      fields: Named(FieldsNamed { ref named, .. }),
      ..
    }) => {
      let f = named_fields_public(named);
      generate_named_output(name, f)
    }
    // unnamed fields
    Struct(DataStruct {
      fields: Unnamed(FieldsUnnamed { ref unnamed, .. }),
      ..
    }) => {
      let f = unnamed_fields_public(unnamed);
      generate_unnamed_output(name, f)
    }
    // enums
    Enum(DataEnum { ref variants, .. }) => generate_enum_output(name, variants),
    _ => unimplemented!("only works for structs and enums"),
  };

  quote!(
      #(#attributes)*
      #basic_output
  )
  .into()
}
