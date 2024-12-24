use proc_macro::TokenStream;
use quote::{format_ident, quote};
// use syn::__private::{Span, TokenStream2};
use syn::__private::TokenStream2;
// use syn::{parse_macro_input, DataStruct, DeriveInput, FieldsNamed, Ident};
use syn::{parse_macro_input, DataStruct, DeriveInput, FieldsNamed, Ident, Type};
use syn::{Data::Struct, Fields::Named};

fn generated_methods(ast: &DeriveInput) -> Vec<TokenStream2> {
  // eprintln!("{:#?}", &ast);
  let named_fields = match ast.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { ref named, .. }),
      ..
    }) => named,
    _ => unimplemented!("only works for structs with named fields"),
  };

  named_fields
    .iter()
    .map(|field| {
      let field_name: &Ident = field.ident.as_ref().unwrap();
      let type_name: &Type = &field.ty;
      // let method_name = Ident::new(&format!("get_{field_name}"), Span::call_site());
      let method_name = format_ident!("get_{field_name}");

      quote!(
          fn #method_name(&self) -> &#type_name {
              &self.#field_name
          }
      )
    })
    .collect()
}

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
  let struct_as_stream: TokenStream2 = item.clone().into();
  let ast: DeriveInput = parse_macro_input!(item);
  let name = &ast.ident;
  let methods = generated_methods(&ast);

  quote!(
      #struct_as_stream

      impl #name {
          #(#methods)*
      }
  )
  .into()
}
