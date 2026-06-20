use proc_macro2::TokenStream;
use quote::quote;
use syn::{__private::TokenStream2, punctuated::Punctuated};
use syn::{Expr, ExprLit, Field, Ident, Lit, Meta, MetaNameValue, token::Comma};

type Fields = Punctuated<Field, Comma>;

pub fn builder_methods(fields: &Fields) -> Vec<TokenStream> {
  fields
    .iter()
    .map(|f @ Field { ident, ty, .. }| {
      f.attrs
        .iter()
        .find(|&attr| attr.path().is_ident("rename")) // NOTE: here is the "rename" attribute
        .map(|attr| &attr.meta)
        .map(|meta: &Meta| {
          match meta {
            // NOTE: this time we use a NameValue
            Meta::NameValue(MetaNameValue {
              value:
                Expr::Lit(ExprLit {
                  lit: Lit::Str(literal_string),
                  ..
                }),
              ..
            }) => Ident::new(&literal_string.value(), literal_string.span()),
            _ => panic!("expected key and value for rename attribute"),
          }
        })
        .map(|new_fn_name| {
          quote! {
              pub fn #new_fn_name(mut self, input: #ty) -> Self {
                  self.#ident = Some(input);
                  self
              }
          }
        })
        // NOTE: lazy evaluation
        .unwrap_or_else(|| {
          quote! {
              // keep the old function name
              pub fn #ident(mut self, input: #ty) -> Self {
                  self.#ident = Some(input);
                  self
              }
          }
        })
    })
    .collect()
}

pub fn original_struct_setters(fields: &Fields) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|Field { ident, .. }| {
    quote! {
        #ident: self.#ident.expect(concat!("field not set: ", stringify!(#ident)))
    }
  })
}

pub fn builder_init_values(fields: &Fields) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|Field { ident, .. }| {
    quote! { #ident: None }
  })
}

pub fn builder_field_definitions(fields: &Fields) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|Field { ident, ty, .. }| {
    quote! { #ident: Option<#ty> }
  })
}
