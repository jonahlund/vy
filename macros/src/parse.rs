use syn::parse::{Parse, ParseStream};
use tiny_rsx::ast::NodeTree;

use crate::{LazyInput, WriteInput};

impl Parse for LazyInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut input_str = input.to_string();
        input_str.retain(|s| !s.is_whitespace());
        let size_hint = input_str.len();

        Ok(Self {
            nodes: NodeTree::parse(input)?.nodes,
            size_hint,
        })
    }
}

impl Parse for WriteInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let buffer = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let LazyInput { nodes, size_hint } = LazyInput::parse(input)?;

        Ok(Self {
            buffer,
            nodes,
            size_hint,
        })
    }
}
