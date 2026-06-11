use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{ToTokens, quote};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Error, Token, parse_macro_input};

#[derive(Debug)]
struct ComposeInput {
  expressions: Punctuated<Ident, Token!(.)>,
}

impl Parse for ComposeInput {
  fn parse(input: ParseStream) -> Result<Self, Error> {
    Ok(ComposeInput {
      expressions: Punctuated::<Ident, Token!(.)>::parse_terminated(input).unwrap(),
    })
  }
}

// NOTE: Composing functions with only one function does not work.
impl ToTokens for ComposeInput {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let mut total: Option<TokenStream2> = None;
    let mut as_idents: Vec<&Ident> = self.expressions.iter().collect();
    // eprintln!("{:?}", as_idents.len());
    let last_fun = as_idents.pop().unwrap();

    as_idents.iter().rev().for_each(|fun| match &total {
      None => {
        total = Some(quote!(
          compose_two(#fun, #last_fun)
        ));
      }
      Some(current) => {
        total = Some(quote!(
          compose_two(#fun, #current)
        ));
      }
    });
    total.to_tokens(tokens);
  }
}

#[proc_macro]
pub fn compose(item: TokenStream) -> TokenStream {
  let ci: ComposeInput = parse_macro_input!(item);
  // eprintln!("{:#?}", ci);

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
