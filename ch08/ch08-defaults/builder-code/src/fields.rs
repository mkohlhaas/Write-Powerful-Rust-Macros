use quote::quote;
use syn::__private::TokenStream2;
use syn::{token::Comma, Expr, ExprLit, Field, Ident, Lit, Meta, MetaNameValue};

type Fields = syn::punctuated::Punctuated<Field, Comma>;

pub fn builder_methods(fields: &Fields) -> Vec<TokenStream2> {
  fields
    .iter()
    .map(|f @ Field { ident, ty, .. }| {
      f.attrs
        .iter()
        .find(|&attr| attr.path().is_ident("rename"))
        .map(|attr| &attr.meta)
        .map(|meta| match meta {
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
              pub fn #attr(mut self, input: #ty) -> Self {
                  self.#ident = Some(input);
                  self
              }
          }
        })
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

// Returning a Vector to make things easier for us (instead of impl..., or Map...).
pub fn original_struct_setters(fields: &Fields, is_using_defaults: bool) -> Vec<TokenStream2> {
  fields
    .iter()
    .map(|Field { ident, .. }| {
      let field_name = stringify!(#ident);

      let handle_type: TokenStream2 = if is_using_defaults {
        default_fallback()
      } else {
        panic_fallback(field_name)
      };

      quote! {
          #ident: self.#ident.#handle_type
      }
    })
    .collect()
}

fn default_fallback() -> TokenStream2 {
  quote! {
      unwrap_or_default()
  }
}

fn panic_fallback(field_name: &str) -> TokenStream2 {
  quote! {
      expect(concat!("Field not set: ", #field_name))
  }
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
