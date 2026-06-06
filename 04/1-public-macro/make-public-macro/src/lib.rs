use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{parse_macro_input, DataStruct, DeriveInput, FieldsNamed};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(item);

  // print out AST
  eprintln!("{:#?}", &ast);

  let name = ast.ident;

  let fields = match ast.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { ref named, .. }),
      ..
    }) => named,
    _ => unimplemented!("only works for structs with named fields"),
  };

  // builder_fields: Map<Iter<'_, Field>, impl FnMut(&Field) -> TokenStream>
  let builder_fields = fields.iter().map(|f| {
    // Option<Ident> uses ToTokens trait which uses ToTokens of its content (Ident)
    let name: &Option<syn::Ident> = &f.ident;
    // eprintln!("{:#?}", &name);

    // syn::Type also uses ToTokens trait
    let ty = &f.ty;
    // eprintln!("{:#?}", &ty);

    // let fu = quote! { pub #name: #ty };
    // eprintln!("{:#?}", &fu);
    // fu

    quote! { pub #name: #ty }
  });

  // Mapping over builder_fields containing Field's.
  // Of course Field implements ToToken trait.
  quote! {
    pub struct #name {
      #(#builder_fields,)*
    }
  }
  .into()
}
