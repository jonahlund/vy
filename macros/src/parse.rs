use syn::parse::{Parse, ParseStream};
use tiny_rsx::Parser;

use crate::{LazyInput, WriteInput};

impl Parse for LazyInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let size_hint = input.to_string().len();
        let mut nodes = Vec::new();
        let parser = Parser::new();
        while !input.is_empty() {
            nodes.push(parser.parse_node(input)?);
        }
        Ok(Self { nodes, size_hint })
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
