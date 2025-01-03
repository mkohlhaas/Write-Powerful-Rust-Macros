use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::token::FatArrow;

struct ComposeInput {
  expressions: Punctuated<Ident, FatArrow>,
}

impl Parse for ComposeInput {
  fn parse(input: ParseStream) -> Result<Self, syn::Error> {
    Ok(ComposeInput {
      expressions: Punctuated::<Ident, FatArrow>::parse_terminated(input).unwrap(),
    })
  }
}

impl ToTokens for ComposeInput {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let mut total = None;
    let mut as_idents: Vec<&Ident> = self.expressions.iter().collect();
    let last_ident = as_idents.pop().unwrap();

    as_idents.iter().rev().for_each(|i| match &total {
      None => {
        total = Some(quote!(
            compose_two(#i, #last_ident)
        ));
      }
      Some(current_total) => {
        total = Some(quote!(
            compose_two(#i, #current_total)
        ));
      }
    });
    total.to_tokens(tokens);
  }
}

#[proc_macro]
pub fn compose(item: TokenStream) -> TokenStream {
  let ci: ComposeInput = parse_macro_input!(item);

  quote!(
      {
          fn compose_two<FIRST, SECOND, THIRD, F, G>(first: F, second: G)
          -> impl Fn(FIRST) -> THIRD
          where
              F: Fn(FIRST) -> SECOND,
              G: Fn(SECOND) -> THIRD,
          {
              move |x| second(first(x))
          }
          #ci
      }
  )
  .into()
}
