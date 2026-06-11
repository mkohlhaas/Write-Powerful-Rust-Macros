use proc_macro::TokenStream;
use quote::quote;
use syn::__private::{Span, TokenStream2};
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{DataStruct, DeriveInput, FieldsNamed, Ident, Type, parse_macro_input};

fn get_field_info(ast: &DeriveInput) -> Vec<(&Ident, &Type)> {
  match ast.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { ref named, .. }),
      ..
    }) => named,
    _ => unimplemented!("only works for structs with named fields"),
  }
  .iter()
  .map(|field| {
    let field_name = field.ident.as_ref().unwrap();
    let type_name = &field.ty;

    (field_name, type_name)
  })
  .collect()
}

fn generated_methods(fields: &Vec<(&Ident, &Type)>) -> Vec<TokenStream2> {
  fields
    .iter()
    .map(|field_info| {
      let (field_name, type_name) = field_info;
      let method_name = Ident::new(&format!("get_{field_name}"), Span::call_site());

      quote!(
          pub fn #method_name(&self) -> &#type_name {
              &self.#field_name
          }
      )
    })
    .collect()
}

fn generate_private_fields(fields: &Vec<(&Ident, &Type)>) -> Vec<TokenStream2> {
  fields
    .iter()
    .map(|(field_name, r#type)| {
      quote!(
          #field_name: #r#type
      )
    })
    .collect()
}

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(item as DeriveInput);
  let name = &ast.ident;
  let fields: Vec<(&Ident, &Type)> = get_field_info(&ast);
  let output_fields = generate_private_fields(&fields);
  let methods = generated_methods(&fields);

  quote!(
      pub struct #name {
          #(#output_fields,)*
      }

      impl #name {
          pub fn new() -> Self {
              #name {
                  string: "String".to_string(),
                  number: 42,
              }
          }

          #(#methods)*
      }
  )
  .into()
}
