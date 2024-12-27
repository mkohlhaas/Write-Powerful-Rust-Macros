use quote::quote;
use syn::{__private::TokenStream2, punctuated::Punctuated};
use syn::{token::Comma, Field, Ident, LitStr, Meta};

type Fields = Punctuated<Field, Comma>;

pub fn builder_methods(fields: &Fields) -> Vec<TokenStream2> {
  fields
    .iter()
    .map(|f @ Field { ident, ty, .. }| {
      let renamed_fn: Option<Ident> = f
        .attrs
        .iter()
        .find(|&attr| attr.path().is_ident("rename")) // NOTE: "rename" attribute is here
        .map(|attr| &attr.meta)
        .map(|meta| match meta {
          Meta::List(nested) => {
            let a: LitStr = nested.parse_args().unwrap();
            Ident::new(&a.value(), a.span())
          }
          Meta::Path(_) => panic!("expected rename to have brackets with name of property"),
          Meta::NameValue(_) => panic!("did not expect rename to have names and values"),
        });

      if let Some(renamed_fn) = renamed_fn {
        quote! {
            pub fn #renamed_fn(mut self, input: #ty) -> Self {
                self.#ident = Some(input);
                self
            }
        }
      } else {
        quote! {
            pub fn #ident(mut self, input: #ty) -> Self {
                self.#ident = Some(input);
                self
            }
        }
      }
    })
    .collect()
}

pub fn original_struct_setters(fields: &Fields) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|Field { ident, .. }| {
    quote! {
        #ident: self.#ident
            .expect(concat!("field not set: ", stringify!(#ident)))
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
