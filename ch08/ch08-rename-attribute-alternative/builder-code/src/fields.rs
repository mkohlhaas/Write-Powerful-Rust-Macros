use proc_macro2::TokenStream;
use quote::quote;
use syn::{__private::TokenStream2, punctuated::Punctuated};
use syn::{token::Comma, Expr, ExprLit, Field, Ident, Lit, Meta, MetaNameValue};

type Fields = Punctuated<Field, Comma>;

pub fn builder_methods(fields: &Fields) -> Vec<TokenStream> {
  fields
    .iter()
    .map(|f @ Field { ident, ty, .. }| {
      f.attrs
        .iter()
        .find(|&attr| attr.path().is_ident("rename")) // NOTE: here is the "rename" attribute
        .map(|attr| &attr.meta)
        .map(|meta| {
          // eprintln!("{:#?}", &meta);
          match meta {
            Meta::NameValue(MetaNameValue {
              value:
                Expr::Lit(ExprLit {
                  lit: Lit::Str(literal_string),
                  ..
                }),
              ..
            }) => {
              // eprintln!("Literal String: {:#?}", &literal_string);
              Ident::new(&literal_string.value(), literal_string.span())
            }
            _ => panic!("expected key and value for rename attribute"),
          }
        })
        .map(|attr| {
          quote! {
              pub fn #attr(mut self, input: #ty) -> Self {
                  self.#ident = Some(input);
                  self
              }
          }
        })
        // NOTE: lazy evaluation
        .unwrap_or_else(|| {
          quote! {
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
