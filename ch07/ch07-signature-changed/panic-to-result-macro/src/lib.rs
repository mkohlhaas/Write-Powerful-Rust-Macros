use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{ItemFn, ReturnType, Stmt, __private::TokenStream2, parse2};

fn signature_output_as_result(item_fn: &ItemFn) -> ReturnType {
  let output: TokenStream2 = match item_fn.sig.output {
    ReturnType::Default => {
      quote! {
          -> Result<(), String>
      }
    }
    ReturnType::Type(_, ref ty) => {
      quote! {
          -> Result<#ty, String>
      }
    }
  };
  parse2(output).unwrap()
}

fn last_statement_as_result(last_statement: Option<Stmt>) -> Stmt {
  let last_stmt = last_statement.unwrap();
  let last_stmt = quote! {
      Ok(#last_stmt)
  };
  // remove semicolon from last statement making it an expression
  Stmt::Expr(parse2(last_stmt).unwrap(), None)
}

#[proc_macro_attribute]
pub fn panic_to_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let mut item_fn: ItemFn = syn::parse(item).unwrap();

  item_fn.sig.output = signature_output_as_result(&item_fn);
  let last_statement: Option<Stmt> = item_fn.block.stmts.pop();
  item_fn
    .block
    .stmts
    .push(last_statement_as_result(last_statement));

  item_fn.to_token_stream().into()
}
