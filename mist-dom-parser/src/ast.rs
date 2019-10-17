use proc_macro::{TokenStream, TokenTree, Ident, Punct, Spacing, Group, Delimiter};

#[derive(Clone, Debug)]
enum TagInfo {
    TagOpen {
        name: Ident,
        properties: Vec<()>,
        closed: bool,
    },
    TagClose {
        name: Ident,
    },
}
