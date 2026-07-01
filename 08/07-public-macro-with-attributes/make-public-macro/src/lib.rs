#![allow(dead_code, unused_features)]
#![feature(iter_collect_into)]

// Rust has six types of attributes:
// https://docs.rs/syn/latest/syn/struct.Attribute.html#syntax
// https://docs.rs/syn/latest/syn/struct.Attribute.html#method.parse_nested_meta

use proc_macro::TokenStream;

use quote::quote;
use syn::meta::ParseNestedMeta;
use syn::parse::{ParseStream, Parser};
use syn::{Data::Struct, Field, Fields::Named, punctuated::Punctuated};
use syn::{DataStruct, DeriveInput, FieldsNamed, Ident, MetaList, Token, parse_macro_input};

const EXCLUDE_ATTRIBUTE_NAME: &str = "exclude";

// Collect excluded fields into basically a vector.
// Will use parse_nested_meta(…).
#[derive(Default)]
struct AlternativeExcludedFields {
  fields: Vec<String>,
}

impl AlternativeExcludedFields {
  fn parse(&mut self, meta: ParseNestedMeta) -> Result<(), syn::Error> {
    println!("`parse` will be called only once!");
    if meta.path.is_ident(EXCLUDE_ATTRIBUTE_NAME) {
      meta.parse_nested_meta(|meta| {
        print!("Will be called for each attribute: ");
        let ident = &meta.path.segments.first().unwrap().ident;
        println!("{}", ident);
        self.fields.push(ident.to_string());
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
      .map(|string| self.fields.contains(&string))
      .unwrap_or(false)
  }
}

// NOT USED
#[derive(Default)]
struct ExcludedFields {
  fields: Vec<String>,
}

// parsing by hand (without parse_nested_meta(…))
// NOT USED
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
      .map(|a_string| self.fields.contains(&a_string))
      .unwrap_or(false)
  }
}

#[proc_macro_attribute]
pub fn public(attr: TokenStream, item: TokenStream) -> TokenStream {
  println!("Attributes: {:?}", attr);
  // println!("Item: {:?}", item);
  let derive_input = parse_macro_input!(item as DeriveInput);
  let mut excluded_fields = AlternativeExcludedFields::default();
  let attr_parser = syn::meta::parser(|meta: ParseNestedMeta<'_>| excluded_fields.parse(meta));
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

  // Debug Code:
  // let builder_fields_clone = builder_fields.clone();
  // let bf = quote! {
  //           #(#builder_fields_clone,)*
  // };
  // println!("Builder Fields: {}", bf);

  let public_version = quote! {
      pub struct #name {
          #(#builder_fields,)*
      }
  };

  println!("{}", public_version);

  public_version.into()
}
