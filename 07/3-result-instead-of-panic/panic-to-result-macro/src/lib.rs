use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
  __private::TokenStream2, Expr, ItemFn, ReturnType, Stmt, StmtMacro, parse2, token::Semi,
};

fn signature_output_as_result(item_fn: &ItemFn) -> ReturnType {
  let output = match item_fn.sig.output {
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
  let last_unwrapped: Stmt = last_statement.unwrap();
  let last_modified: TokenStream2 = quote! {
      Ok(#last_unwrapped)
  };
  Stmt::Expr(parse2(last_modified).unwrap(), None)
}

// check if's
fn handle_expression(expression: Expr, token: Option<Semi>) -> Stmt {
  if let Expr::If(mut ex_if) = expression {
    let new_statements: Vec<Stmt> = ex_if
      .then_branch
      .stmts
      .into_iter()
      .map(|stmt| {
        if let Stmt::Macro(ref expr_macro) = stmt {
          extract_panic_content(expr_macro)
            .map(|ts| {
              quote! {
                  return Err(#ts.to_string());
              }
            })
            .map(parse2)
            .map(Result::unwrap) // unwrap Result of parse2
            .unwrap() // unwrap Option
        } else {
          stmt
        }
      })
      .collect();
    ex_if.then_branch.stmts = new_statements;
    Stmt::Expr(Expr::If(ex_if), token)
  } else {
    Stmt::Expr(expression, token)
  }
}

// Returns an optional message from `panic!`.
// Will be used as the error message in `Result`.
fn extract_panic_content(stmt_macro: &StmtMacro) -> Option<TokenStream2> {
  let does_panic = stmt_macro
    .mac
    .path
    .segments
    .iter()
    .any(|v| v.ident.to_string().eq("panic"));

  if does_panic {
    Some(stmt_macro.mac.tokens.clone()) // `tokens` is the panic msg
  } else {
    None
  }
}

#[proc_macro_attribute]
pub fn panic_to_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let mut item_fn: ItemFn = syn::parse(item).unwrap();

  let new_statements: Vec<Stmt> = item_fn
    .block
    .stmts
    .into_iter()
    .map(|s| {
      if let Stmt::Expr(e, t) = s {
        handle_expression(e, t)
      } else {
        s
      }
    })
    .collect();
  item_fn.block.stmts = new_statements;

  item_fn.sig.output = signature_output_as_result(&item_fn);
  let last_statement = item_fn.block.stmts.pop();
  item_fn
    .block
    .stmts
    .push(last_statement_as_result(last_statement));

  item_fn.to_token_stream().into()
}
