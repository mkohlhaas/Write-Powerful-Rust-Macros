use proc_macro2::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Expr, ExprLit, Field, Ident, Lit, Meta, MetaNameValue, Type};

pub fn builder_methods(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream> {
  fields
    .iter()
    .map(|f| {
      let (field_name, field_type) = get_name_and_type(f);

      extract_attribute_from_field(f, "rename")
        .map(|a| &a.meta)
        .map(|m| match m {
          Meta::NameValue(MetaNameValue {
            value:
              Expr::Lit(ExprLit {
                lit: Lit::Str(literal_string),
                ..
              }),
            ..
          }) => Ident::new(&literal_string.value(), literal_string.span()),
          _ => panic!("expected key and value for rename attribute"),
        })
        .map(|attr| {
          quote! {
              pub fn #attr(mut self, input: #field_type) -> Self {
                  self.#field_name = Some(input);
                  self
              }
          }
        })
        .unwrap_or_else(|| {
          quote! {
              pub fn #field_name(mut self, input: #field_type) -> Self {
                  self.#field_name = Some(input);
                  self
              }
          }
        })
    })
    .collect()
}

pub fn original_struct_setters(
  fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|f| {
    let field_name = &f.ident;
    let field_name_as_string = field_name.as_ref().unwrap().to_string();

    quote! {
        #field_name: self.#field_name
            .expect(concat!("field not set: ", #field_name_as_string))
    }
  })
}

pub fn builder_init_values(
  fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|f| {
    let field_name = &f.ident;
    quote! { #field_name: None }
  })
}

pub fn builder_field_definitions(
  fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|f| {
    let (field_name, field_type) = get_name_and_type(f);
    quote! { #field_name: Option<#field_type> }
  })
}

fn get_name_and_type<'a>(f: &'a Field) -> (&'a Option<Ident>, &'a Type) {
  let field_name = &f.ident;
  let field_type = &f.ty;
  (field_name, field_type)
}

fn extract_attribute_from_field<'a>(f: &'a Field, name: &'a str) -> Option<&'a syn::Attribute> {
  f.attrs.iter().find(|&attr| attr.path().is_ident(name))
}
