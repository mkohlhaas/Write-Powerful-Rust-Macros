use quote::quote;
use syn::{Field, __private::TokenStream2, punctuated::Punctuated, token::Comma};

pub fn original_struct_setters(
  fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|Field { ident, .. }| {
    quote! {
        #ident: self.#ident
            .expect(concat!("field not set: ", stringify!(#ident)))
    }
  })
}

pub fn builder_methods(
  fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|Field { ident, ty, .. }| {
    quote! {
        pub fn #ident(mut self, input: #ty) -> Self {
            self.#ident = Some(input);
            self
        }
    }
  })
}

pub fn builder_init_values(
  fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|Field { ident, .. }| {
    quote! { #ident: None }
  })
}

pub fn builder_field_definitions(
  fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|Field { ident, ty, .. }| {
    quote! { #ident: Option<#ty> }
  })
}

#[cfg(test)]
mod tests {
  use proc_macro2::Span;
  use syn::{FieldMutability, Ident, Path, PathSegment, Type, TypePath, Visibility};

  use super::*;

  #[test]
  fn get_name_and_type_give_back_name() {
    let p = PathSegment {
      ident: Ident::new("String", Span::call_site()),
      arguments: Default::default(),
    };

    let mut pun = Punctuated::new();
    pun.push(p);

    let ty = Type::Path(TypePath {
      qself: None,
      path: Path {
        leading_colon: None,
        segments: pun,
      },
    });

    let f = Field {
      attrs: vec![],
      vis: Visibility::Inherited,
      mutability: FieldMutability::None,
      ident: Some(Ident::new("example", Span::call_site())),
      colon_token: None,
      ty,
    };

    let actual_name = &f.ident;

    assert_eq!(
      actual_name.as_ref().unwrap().to_string(),
      "example".to_string()
    )
  }
}
