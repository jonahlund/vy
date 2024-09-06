use syn::parse::{Parse, ParseStream};
use tiny_rsx::parse::Parser;

use crate::{LazyInput, RenderInput, WriteInput};

impl Parse for LazyInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut input_string = input.to_string();
        let mut nodes = Vec::new();
        let parser = Parser::new();
        while !input.is_empty() {
            nodes.push(parser.parse_node(input)?);
        }
        Ok(Self {
            size_hint: rm_whitespace(&mut input_string).len(),
            nodes,
        })
    }
}

impl Parse for WriteInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let buffer = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let LazyInput { size_hint, nodes } = LazyInput::parse(input)?;

        Ok(Self {
            buffer,
            size_hint,
            nodes,
        })
    }
}

impl Parse for RenderInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let LazyInput { size_hint, nodes } = LazyInput::parse(input)?;

        Ok(Self { size_hint, nodes })
    }
}

fn rm_whitespace(s: &mut String) -> &mut String {
    s.retain(|c| !c.is_whitespace());
    s
}
