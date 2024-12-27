use quote::quote;
use syn::__private::TokenStream2;
use syn::{token::Comma, Field, Ident, LitStr, Token, Type};

type PFields = syn::punctuated::Punctuated<Field, Comma>;

pub fn builder_methods(fields: &PFields) -> Vec<TokenStream2> {
  fields
    .iter()
    .map(|field| {
      let (field_name, field_type) = get_name_and_type(field);
      let attr = extract_attribute_from_field(field, "builder").map(|attr| {
        let mut content = None;

        // eprintln!("{:#?}", attr);
        attr
          .parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
              let _: Token![=] = meta.input.parse().unwrap();
              let name: LitStr = meta.input.parse().unwrap();
              content = Some(Ident::new(&name.value(), name.span()));
            }
            Ok(())
          })
          .unwrap();
        content.unwrap()
      });

      if let Some(attr) = attr {
        quote! {
            pub fn #attr(mut self, input: #field_type) -> Self {
                self.#field_name = Some(input);
                self
            }
        }
      } else {
        quote! {
            pub fn #field_name(mut self, input: #field_type) -> Self {
                self.#field_name = Some(input);
                self
            }
        }
      }
    })
    .collect()
}

pub fn original_struct_setters(fields: &PFields) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|f| {
    let field_name = &f.ident;
    let field_name_as_string = field_name.as_ref().unwrap().to_string();

    quote! {
        #field_name: self.#field_name
            .expect(concat!("field not set: ", #field_name_as_string))
    }
  })
}

pub fn builder_init_values(fields: &PFields) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|f| {
    let field_name = &f.ident;
    quote! { #field_name: None }
  })
}

pub fn builder_field_definitions(fields: &PFields) -> impl Iterator<Item = TokenStream2> + '_ {
  fields.iter().map(|f| {
    let (field_name, field_type) = get_name_and_type(f);
    quote! { #field_name: Option<#field_type> }
  })
}

fn get_name_and_type(Field { ident, ty, .. }: &Field) -> (&Option<Ident>, &Type) {
  (ident, ty)
}

fn extract_attribute_from_field<'a>(f: &'a Field, name: &'a str) -> Option<&'a syn::Attribute> {
  f.attrs.iter().find(|&attr| attr.path().is_ident(name))
}
