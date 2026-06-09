use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Colon;
use syn::{Data::Struct, DataStruct, Fields::Named, FieldsNamed};
use syn::{DeriveInput, Ident, Visibility, parse_macro_input};

struct StructField {
  name: Ident,
  ty: Ident,
}

impl ToTokens for StructField {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let n = &self.name;
    let t = &self.ty;
    quote!(pub #n: #t).to_tokens(tokens)
  }
}

impl Parse for StructField {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    // eprintln!("{:#?}", input);
    let _vis: Result<Visibility, _> = input.parse();
    let list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();

    // eprintln!("Punctuated length: {:#?}", list.len()); // 2
    // eprintln!("{:#?}", _vis);
    // eprintln!("{:#?}", list);

    Ok(StructField {
      name: list.first().unwrap().clone(),
      ty: list.last().unwrap().clone(),
    })
  }
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(item as DeriveInput);
  // eprintln!("{:#?}", &ast);
  let name = ast.ident;

  let fields = match ast.data {
    Struct(DataStruct {
      fields: Named(FieldsNamed { ref named, .. }),
      ..
    }) => named,
    _ => unimplemented!("only works for structs with named fields"),
  };

  // NOTE: Implements notable traits: `Iterator<Item = StructField>`
  //
  // NOTE: is an iterator of StructField's
  let builder_fields = fields.iter().map(|field| {
    // eprintln!("{:#?}", f.to_token_stream());
    syn::parse2::<StructField>(field.to_token_stream()).unwrap()
  });

  quote! {
      pub struct #name {
          #(#builder_fields,)*
      }
  }
  .into()
}
