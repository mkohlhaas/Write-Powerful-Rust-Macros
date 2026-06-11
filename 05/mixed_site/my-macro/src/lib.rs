use proc_macro::TokenStream;
use quote::quote;
use syn::__private::Span;
use syn::Ident;

// difference `call_site(…)`, `mixed_site(…)`, `def_site(…)`

#[proc_macro]
pub fn local(_item: TokenStream) -> TokenStream {
  let greeting = Ident::new("greeting", Span::call_site());
  // let greeting = Ident::new("greeting", Span::mixed_site());
  // let greeting = Ident::new("greeting", Span::def_site()); // this is a nightly-only experimental API (

  dbg!(&greeting);

  quote!(let #greeting = "Heya! It's me, John!";).into()
}
