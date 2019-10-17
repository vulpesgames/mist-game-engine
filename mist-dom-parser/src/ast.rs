#![allow(dead_code)]

use proc_macro::{TokenStream, TokenTree, Ident, Literal, Punct, Spacing, Group, Delimiter};

#[derive(Clone, Debug)]
pub enum Property {
    Flag(Ident),
    Literal(Ident, Literal),
    Block(Ident, Group),
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub name: Ident,
    pub properties: Vec<Property>,
    pub closed: bool,
    pub slots: Vec<Tag>,
}

#[derive(Clone, Debug)]
pub enum AstNode {
    Tag(Tag),
    Container(Vec<Tag>),
}
