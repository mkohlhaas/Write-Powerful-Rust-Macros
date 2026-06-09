use proc_macro::TokenStream;

use quote::{ToTokens, quote};
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{DataStruct, DeriveInput, Field, FieldsNamed, Ident, Type, parse_macro_input};

struct StructField {
  // name: Ident,
  //
  // NOTE: Option's ToTokens trait implementation directs to Ident's ToTokens implementation
  name: Option<Ident>,
  ty: Type,
}

impl StructField {
  fn new(field: &Field) -> Self {
    Self {
      // Funny business not necessary.
      // name: field.ident.as_ref().unwrap().clone(),
      name: field.ident.clone(),
      ty: field.ty.clone(),
    }
  }
}

impl ToTokens for StructField {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let n = &self.name;
    let t = &self.ty;
    quote!(pub #n: #t).to_tokens(tokens)
  }
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(item as DeriveInput);
  let name = ast.ident;

  let fields = match ast.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { ref named, .. }),
      ..
    }) => named,
    _ => unimplemented!("only works for structs with named fields"),
  };

  // NOTE: - point-free mapping (instead of map(|f| StructField::new(f)))
  //       - Implements notable traits: `Iterator<Item = StructField>`
  //
  // NOTE: is an iterator over StructField's
  let builder_fields = fields.iter().map(StructField::new);

  quote! {
      pub struct #name {
          #(#builder_fields,)*
      }
  }
  .into()
}
