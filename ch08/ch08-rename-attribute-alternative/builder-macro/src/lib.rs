use builder_code::create_builder;
use proc_macro::TokenStream;

#[proc_macro_derive(Builder, attributes(rename))]
pub fn builder(item: TokenStream) -> TokenStream {
  create_builder(item.into()).into()
}
