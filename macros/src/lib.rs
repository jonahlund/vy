mod fmt;

use fmt::Formatter;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens as _};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Path, Token,
};
use tiny_rsx::ast::{Node, NodeTree};

struct Into {
    makro: Path,
    nodes: Box<[Node]>,
}

impl Parse for Into {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let makro = input.parse()?;
        _ = input.parse::<Token![,]>();
        let nodes = input.parse::<NodeTree>()?.0;

        Ok(Self { makro, nodes })
    }
}

#[proc_macro]
pub fn into(input: TokenStream) -> TokenStream {
    let Into { makro, nodes } = parse_macro_input!(input as Into);

    let mut fmt = Formatter::new();
    for node in &nodes {
        fmt.write_node(node);
    }
    let parts = into_parts(&fmt.literal, &fmt.values);

    quote! {
        #makro!(#(#parts),*)
    }
    .into()
}

struct InsertAt<'a>(usize, &'a Expr);

fn into_parts(literal: &str, values: &[InsertAt]) -> Vec<TokenStream2> {
    let mut res = Vec::new();
    let mut cursor = 0;
    for InsertAt(pos, expr) in values {
        let slice = &literal[cursor..*pos];
        if !slice.is_empty() {
            res.push(slice.to_token_stream());
        }
        res.push(expr.to_token_stream());
        cursor = *pos;
    }
    if cursor < literal.len() {
        let slice = &literal[cursor..];
        res.push(slice.to_token_stream());
    }
    res
}
