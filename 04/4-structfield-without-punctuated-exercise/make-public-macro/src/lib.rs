use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::token::Colon;
use syn::{Data::Struct, DataStruct};
use syn::{DeriveInput, Ident, Type, Visibility, parse_macro_input};
use syn::{Fields::Named, FieldsNamed};

type TokenStream2 = proc_macro2::TokenStream;

struct StructField {
  name: Ident,
  ty: Type,
}

impl ToTokens for StructField {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let n = &self.name;
    let t = &self.ty;
    quote!(pub #n: #t).to_tokens(tokens)
  }
}

impl Parse for StructField {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let _vis: Result<Visibility, _> = input.parse();
    let name: Ident = input.parse().unwrap();
    let _colon: Result<Colon, _> = input.parse();
    let ty: Type = input.parse().unwrap();

    Ok(StructField { name, ty })
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

  // NOTE: Implements notable traits: `Iterator<Item = StructField>`
  let builder_fields = fields
    .iter()
    .map(|field| syn::parse2::<StructField>(field.to_token_stream()).unwrap());

  let public_version = quote! {
      pub struct #name {
          #(#builder_fields,)*
      }
  };

  public_version.into()
}
