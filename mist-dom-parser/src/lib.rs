extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree, Ident, Punct, Spacing, Group, Delimiter};
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, Expr};

mod ast;
mod lexer;
mod parser;
mod codegen;

#[proc_macro_hack]
pub fn render(input: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();
    
    // unroll stream
    let tokens = lexer::unroll_token(input);
    println!("{:#?}", tokens);

    output.extend(vec![
        TokenTree::Group(
            Group::new(Delimiter::Brace, TokenStream::new())
        )
    ].into_iter());

    output
}
