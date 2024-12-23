use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Colon;
use syn::{parse_macro_input, DeriveInput, Ident, Visibility};
use syn::{Data::Struct, DataStruct, Fields::Named, FieldsNamed};

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

  // builder_fields: Map<Iter<'_, Field>, impl FnMut(&Field) -> StructField>
  let builder_fields = fields.iter().map(|f| {
    // eprintln!("{:#?}", f.to_token_stream());
    syn::parse2::<StructField>(f.to_token_stream()).unwrap()
  });

  quote! {
      pub struct #name {
          #(#builder_fields,)*
      }
  }
  .into()
}
