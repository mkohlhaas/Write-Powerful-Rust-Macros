use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::FatArrow;
use syn::{Error, parse_macro_input};

struct ComposeInput {
  expressions: Punctuated<Ident, FatArrow>,
}

impl Parse for ComposeInput {
  fn parse(input: ParseStream) -> Result<Self, Error> {
    Ok(ComposeInput {
      // => (aka FatArrow; Token!(=>) should return the result of the FatArrow)
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
          fn compose_two<F, G, H, Fn1, Fn2>(first: Fn1, second: Fn2)
          -> impl Fn(F) -> H
          where
              Fn1: Fn(F) -> G,
              Fn2: Fn(G) -> H,
          {
              move |x| second(first(x))
          }

          #ci
      }
  )
  .into()
}
