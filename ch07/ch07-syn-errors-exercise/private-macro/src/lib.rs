use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Enum;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::__private::{Span, ToTokens, TokenStream2};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DataStruct, DeriveInput, FieldsNamed, Ident, Type};

fn get_field_info(ast: &DeriveInput) -> Result<Vec<(&Ident, &Type)>, syn::Error> {
  Ok(
    match ast.data {
      Struct(DataStruct {
        fields: Named(FieldsNamed { ref named, .. }),
        ..
      }) => named,
      Enum(ref d) => {
        return Err(syn::Error::new(
          d.enum_token.span(),
          "does not work for enums",
        ))
      }
      _ => {
        return Err(syn::Error::new(
          ast.ident.span(),
          "only works for structs with named fields",
        ))
      }
    }
    .iter()
    .map(|f| {
      let field_name = f.ident.as_ref().take().unwrap();
      let type_name = &f.ty;

      (field_name, type_name)
    })
    .collect(),
  )
}

fn generated_methods(fields: &Vec<(&Ident, &Type)>) -> Vec<TokenStream2> {
  fields
    .iter()
    .map(|f| {
      let (field_name, type_name) = f;
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
    .map(|f| {
      let (field_name, type_name) = f;

      quote!(
          #field_name: #type_name
      )
    })
    .collect()
}

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(item as DeriveInput);
  let name = &ast.ident;

  let fields = match get_field_info(&ast) {
    Ok(fields) => fields,
    Err(err) => return err.to_compile_error().to_token_stream().into(),
  };

  let output_fields = generate_private_fields(&fields);
  let methods = generated_methods(&fields);

  quote!(
      pub struct #name {
          #(#output_fields,)*
      }

      impl #name {
          pub fn new() -> Self {
              Example {
                  string_value: "value".to_string(),
                  number_value: 2,
              }
          }

          #(#methods)*
      }
  )
  .into()
}
