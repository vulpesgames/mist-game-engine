extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree, Punct, Spacing, Group, Delimiter};
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, Expr};

#[proc_macro_hack]
pub fn render(input: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();

    println!("{}", input);
    
    for token in input {
        println!("{:?}", token);

        match token {
            _ => {
                // TODO
            }
        }
    }

    output.extend(vec![
        TokenTree::Group(
            Group::new(Delimiter::Brace, TokenStream::new())
        )
    ].into_iter());

    output
}
