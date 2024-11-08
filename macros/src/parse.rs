use syn::parse::{Parse, ParseStream};
use tiny_rsx::ast::NodeTree;

use crate::{fmt::Formatter, LazyInput, LitInput, WriteInput};

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

impl Parse for LitInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let NodeTree { nodes } = NodeTree::parse(input)?;

        let mut text = String::new();
        let mut args = Vec::new();

        let mut fmt = Formatter::new(&mut text, &mut args);

        for node in &nodes {
            let _ = fmt.write_node(node);
        }

        if let Some((_, value)) = args.into_iter().next() {
            return Err(syn::Error::new_spanned(
                value,
                "only literals allowed",
            ));
        }

        Ok(Self { text })
    }
}
