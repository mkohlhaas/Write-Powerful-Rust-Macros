use builder_code::create_builder;
use proc_macro::TokenStream;

// Tell macro system we are having a "rename" attribute.
#[proc_macro_derive(Builder, attributes(rename))]
pub fn builder(item: TokenStream) -> TokenStream {
  create_builder(item.into()).into()
}
