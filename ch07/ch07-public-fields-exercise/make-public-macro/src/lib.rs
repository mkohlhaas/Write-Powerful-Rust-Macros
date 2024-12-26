use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Visibility};
use syn::{Data::Struct, Fields::Named, __private::ToTokens};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let mut derive_input = parse_macro_input!(item as DeriveInput);

  let fields = if let Struct(DataStruct {
    fields: Named(FieldsNamed { ref named, .. }),
    ..
  }) = derive_input.data
  {
    named
  } else {
    unimplemented!("only works for structs with named fields")
  };

  let builder_fields = fields.iter().map(|field| {
    let name = &field.ident;
    let ty = &field.ty;
    quote! { pub #name: #ty }
  });

  let builder_fields = quote!(
      {
          #(#builder_fields,)*
      }
  );

  derive_input.data = Data::Struct(DataStruct {
    struct_token: Default::default(),
    fields: Fields::Named(syn::parse2(builder_fields).unwrap()),
    semi_token: None,
  });
  derive_input.vis = Visibility::Public(Default::default());

  derive_input.to_token_stream().into()
}
