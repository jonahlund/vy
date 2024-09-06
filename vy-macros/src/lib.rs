use proc_macro::TokenStream;
use syn::parse_macro_input;
use tiny_rsx::ast;

mod expand;
mod parse;

pub(crate) struct LazyInput {
    size_hint: usize,
    nodes: Vec<ast::Node>,
}

#[proc_macro]
pub fn lazy(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LazyInput);

    expand::lazy(input).into()
}

pub(crate) struct WriteInput {
    buffer: syn::Expr,
    size_hint: usize,
    nodes: Vec<ast::Node>,
}

#[proc_macro]
pub fn write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as WriteInput);

    expand::write(input).into()
}

pub(crate) struct RenderInput {
    size_hint: usize,
    nodes: Vec<ast::Node>,
}

#[proc_macro]
pub fn render(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RenderInput);

    expand::render(input).into()
}
