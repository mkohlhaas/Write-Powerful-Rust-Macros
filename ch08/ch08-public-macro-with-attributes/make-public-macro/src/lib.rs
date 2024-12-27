#![allow(dead_code)]

use proc_macro::TokenStream;

use quote::quote;
use syn::meta::ParseNestedMeta;
use syn::parse::{ParseStream, Parser};
use syn::{parse_macro_input, DataStruct, DeriveInput, FieldsNamed, Ident, MetaList, Token};
use syn::{punctuated::Punctuated, Data::Struct, Field, Fields::Named};

const EXCLUDE_ATTRIBUTE_NAME: &str = "exclude";

// Collect excluded fields into basically a vector.
#[derive(Default)]
struct AlternativeExcludedFields {
  fields: Vec<String>,
}

impl AlternativeExcludedFields {
  fn parse(&mut self, meta: ParseNestedMeta) -> Result<(), syn::Error> {
    if meta.path.is_ident(EXCLUDE_ATTRIBUTE_NAME) {
      meta.parse_nested_meta(|meta| {
        let ident = &meta.path.segments.first().unwrap().ident;
        self.fields.push(ident.to_string()); // collection happens here
        Ok(())
      })
    } else {
      Err(meta.error("unsupported property"))
    }
  }

  fn matches_ident(&self, name: &Option<Ident>) -> bool {
    name
      .as_ref()
      .map(|ident| ident.to_string())
      .map(|a_string| {
        self
          .fields
          .iter()
          .any(|other_string| *other_string == a_string)
      })
      .unwrap_or_else(|| false)
  }
}

// NOT USED
#[derive(Default)]
struct ExcludedFields {
  fields: Vec<String>,
}

impl ExcludedFields {
  fn parse(input: ParseStream) -> Result<Self, syn::Error> {
    match input.parse::<MetaList>() {
      Ok(meta_list) => {
        if meta_list
          .path
          .segments
          .iter()
          .any(|path| path.ident == EXCLUDE_ATTRIBUTE_NAME)
        {
          let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
          let identifiers = parser.parse(meta_list.clone().tokens.into()).unwrap();
          let fields = identifiers.iter().map(|ident| ident.to_string()).collect();
          Ok(ExcludedFields { fields })
        } else {
          Ok(ExcludedFields { fields: vec![] })
        }
      }
      Err(_) => Ok(ExcludedFields { fields: vec![] }),
    }
  }

  // same as for AlternativeExcludedFields
  fn matches_ident(&self, name: &Option<Ident>) -> bool {
    name
      .as_ref()
      .map(|ident| ident.to_string())
      .map(|a_string| {
        self
          .fields
          .iter()
          .any(|other_string| *other_string == a_string)
      })
      .unwrap_or_else(|| false)
  }
}

#[proc_macro_attribute]
pub fn public(attr: TokenStream, item: TokenStream) -> TokenStream {
  // eprintln!("Debug: {:#?}", &attr);
  // eprintln!("Debug: {:#?}", &item);
  let derive_input = parse_macro_input!(item as DeriveInput);
  let mut excluded_fields = AlternativeExcludedFields::default();
  let attr_parser = syn::meta::parser(|meta| excluded_fields.parse(meta));
  parse_macro_input!(attr with attr_parser);

  let name = derive_input.ident;

  let fields = match derive_input.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { ref named, .. }),
      ..
    }) => named,
    _ => unimplemented!("only works for structs with named fields"),
  };

  let builder_fields = fields.iter().map(|Field { ident, ty, vis, .. }| {
    if excluded_fields.matches_ident(ident) {
      quote! { #vis #ident: #ty }
    } else {
      quote! { pub #ident: #ty }
    }
  });

  let public_version = quote! {
      pub struct #name {
          #(#builder_fields,)*
      }
  };

  public_version.into()
}
