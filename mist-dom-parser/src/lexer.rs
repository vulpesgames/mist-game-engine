#![allow(dead_code)]

use proc_macro::{TokenStream, TokenTree, Ident, Punct, Span, Literal, Delimiter};

#[derive(Clone, Debug)]
pub(crate) enum Token {
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
    Group {
        delimiter: Delimiter,
        tokens: Vec<Token>,
        span: Span,
    }
}

pub(crate) fn unroll_token(stream: TokenStream) -> Vec<Token> {
    let mut tokens = vec![];
    
    for tok in stream {
        tokens.push(match tok {
            TokenTree::Ident(ident) => {
                Token::Ident(ident)
            }
            TokenTree::Punct(punct) => {
                Token::Punct(punct)
            }
            TokenTree::Literal(literal) => {
                Token::Literal(literal)
            }
            TokenTree::Group(group) => {
                Token::Group {
                    delimiter: group.delimiter(),
                    tokens: unroll_token(group.stream()),
                    span: group.span(),
                }
            }
        });
    }

    tokens
}
