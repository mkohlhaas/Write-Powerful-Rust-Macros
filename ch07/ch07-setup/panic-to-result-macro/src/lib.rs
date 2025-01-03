use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn panic_to_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let ast: ItemFn = parse(item).unwrap();
  ast.to_token_stream().into()
}
