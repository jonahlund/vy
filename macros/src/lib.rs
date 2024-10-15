use proc_macro::TokenStream;
use syn::parse_macro_input;
use tiny_rsx::ast;

mod expand;
mod fmt;
mod parse;

pub(crate) struct LazyInput {
    nodes: Box<[ast::Node]>,
    size_hint: usize,
}

#[proc_macro]
pub fn lazy(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LazyInput);

    expand::lazy(input).into()
}

pub(crate) struct WriteInput {
    buffer: syn::Expr,
    nodes: Box<[ast::Node]>,
    size_hint: usize,
}

#[proc_macro]
pub fn write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as WriteInput);

    expand::write(input).into()
}
