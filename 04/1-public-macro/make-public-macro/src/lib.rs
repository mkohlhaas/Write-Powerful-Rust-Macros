use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{DataStruct, DeriveInput, FieldsNamed, parse_macro_input};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item);

  // print out AST
  eprintln!("{:#?}", ast);

  let name = ast.ident;

  let fields = match ast.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { ref named, .. }), // NOTE: `ref`: don't consume the matched item
      ..
    }) => named,
    _ => unimplemented!("only works for structs with named fields"),
  };

  // NOTE: Implements notable traits: `Iterator<Item = TokenStream>`
  //
  // NOTE: is an iterator over TokenStream's
  let builder_fields = fields.iter().map(|field| {
    // Option<Ident> uses ToTokens trait which uses ToTokens of its content (Ident)
    let name: &Option<syn::Ident> = &field.ident;
    eprintln!("Field: {:#?}", name);

    // syn::Type also uses ToTokens trait
    let ty = &field.ty;
    eprintln!("Field type: {:#?}", ty);

    // make the field public
    quote! { pub #name: #ty }
  });

  // Mapping over builder_fields containing Field's.
  // Field implements ToToken trait, too. ;-)
  quote! {
    pub struct #name {
      #(#builder_fields,)*
    }
  }
  .into()
}
