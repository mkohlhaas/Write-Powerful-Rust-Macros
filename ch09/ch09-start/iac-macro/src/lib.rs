use proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::{Ident, LitInt, Token};

pub(crate) mod kw {
  syn::custom_keyword!(bucket);
  syn::custom_keyword!(lambda);
  syn::custom_keyword!(mem);
  syn::custom_keyword!(time);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Bucket {
  name: String,
  has_event: bool,
}

impl Parse for Bucket {
  fn parse(input: ParseStream) -> Result<Self, syn::Error> {
    let bucket_token = input
      .parse::<kw::bucket>()
      .expect("we just checked for this token");
    let bucket_name = input
      .parse()
      .map(|v: Ident| v.to_string())
      .map_err(|_| syn::Error::new(bucket_token.span(), "bucket needs a name"))?;

    let event_needed = if !input.peek(kw::lambda) && input.peek(Token!(=>)) {
      // by peeking, we know the token is present. and we want to get rid of it
      let _ = input.parse::<Token!(=>)>().unwrap();
      true
    } else {
      false
    };

    Ok(Bucket {
      name: bucket_name,
      has_event: event_needed,
    })
  }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Lambda {
  name: String,
  memory: Option<u16>,
  time: Option<u16>,
}

impl Parse for Lambda {
  fn parse(input: ParseStream) -> Result<Self, syn::Error> {
    let lambda_token = input
      .parse::<kw::lambda>()
      .expect("we just checked for this token");
    let lambda_name = input
      .parse()
      .map(|name: Ident| name.to_string())
      .map_err(|_| syn::Error::new(lambda_token.span, "lambda needs a name"))?;

    let mut lambda_memory = None;
    let mut lambda_timeout = None;

    while !input.is_empty() && !input.peek(kw::bucket) {
      if input.peek(kw::mem) {
        let _ = input
          .parse::<kw::mem>()
          .expect("we just checked for this token");
        lambda_memory = Some(input.parse().map(|v: LitInt| {
          v.to_string().parse().map_err(|_| {
            // LitInt will stop most errors, but not negative values or those that are too big
            syn::Error::new(v.span(), "memory needs a positive value <= 10240")
          })
        })??);
      } else if input.peek(kw::time) {
        let _ = input
          .parse::<kw::time>()
          .expect("we just checked for this token");
        lambda_timeout = Some(input.parse().map(|v: LitInt| {
          v.to_string()
            .parse()
            .map_err(|_| syn::Error::new(v.span(), "timeout needs a positive value <= 900"))
        })??);
      } else {
        Err(syn::Error::new(
          input.span(),
          "unknown property passed to lambda",
        ))?
      }
    }

    Ok(Lambda {
      name: lambda_name,
      memory: lambda_memory,
      time: lambda_timeout,
    })
  }
}

#[allow(dead_code)]
#[derive(Debug)]
struct IacInput {
  bucket: Option<Bucket>,
  lambda: Option<Lambda>,
}

impl Parse for IacInput {
  fn parse(input: ParseStream) -> Result<Self, syn::Error> {
    let mut bucket: Option<Bucket> = None;
    let mut lambda = None;

    loop {
      if input.peek(kw::bucket) {
        bucket = Some(input.parse()?);
      } else if input.peek(kw::lambda) {
        lambda = Some(input.parse()?);
      } else if !input.is_empty() {
        return Err(syn::Error::new(
          input.lookahead1().error().span(),
          "only 'bucket' and 'lambda' resources are supported",
        ));
      } else {
        break; // no input left, stop
      }
    }

    // check if a bucket's event has a required lambda
    if bucket
      .as_ref()
      .map(|bucket| bucket.has_event)
      .unwrap_or(false)
      && lambda.is_none()
    {
      return Err(syn::Error::new(
        input.span(),
        "a lambda is required for an event ('=>')",
      ));
    }

    Ok(IacInput { bucket, lambda })
  }
}

#[proc_macro]
pub fn iac(item: TokenStream) -> TokenStream {
  let ii: IacInput = parse_macro_input!(item);
  eprintln!("{:?}", ii);
  quote!().into()
}
