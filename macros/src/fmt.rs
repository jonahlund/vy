use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_quote, Block, Expr, ExprBlock, ExprGroup, ExprLit, Ident, Lit, Token,
};
use vy_core::{Buffer, IntoHtml};

use crate::{
    ast::{Attr, AttrValue, Element, Node},
    known::is_void_tag,
};

pub struct Serializer<'s> {
    buf: &'s mut Buffer,
    values: Vec<(usize, Expr)>,
    imports: Vec<Ident>,
}

impl<'s> Serializer<'s> {
    pub fn new(buf: &'s mut Buffer) -> Self {
        Self {
            buf,
            values: Vec::new(),
            imports: Vec::new(),
        }
    }

    pub fn write_expr(&mut self, mut expr: Expr) {
        match expr {
            Expr::Group(ExprGroup { attrs, expr, .. }) if attrs.is_empty() => {
                self.write_expr(*expr);
            }
            Expr::If(_) => {
                let mut count = 1;
                let mut current = Some(&expr);
                while let Some(Expr::If(node)) = current {
                    current =
                        node.else_branch.as_ref().map(|(_, expr)| &**expr);
                    count += 1;
                }

                let either_suffix =
                    if count > 2 { Some(count) } else { None }.into_string();
                let either = format_ident!("Either{either_suffix}");

                transform_branches(&either, 0, &mut expr);

                self.values.push((self.buf.len(), expr));
            }
            Expr::Lit(ExprLit {
                attrs,
                lit: Lit::Str(lit_str),
            }) if attrs.is_empty() => {
                lit_str.value().escape_and_write(self.buf);
            }
            Expr::Lit(ExprLit {
                attrs,
                lit: Lit::Char(lit_char),
            }) if attrs.is_empty() => {
                lit_char.value().escape_and_write(self.buf);
            }
            Expr::Lit(ExprLit {
                attrs,
                lit: Lit::Int(lit_int),
            }) if attrs.is_empty() => {
                lit_int.base10_digits().escape_and_write(self.buf);
            }
            Expr::Lit(ExprLit {
                attrs,
                lit: Lit::Float(lit_float),
            }) if attrs.is_empty() => {
                lit_float.base10_digits().escape_and_write(self.buf);
            }
            Expr::Lit(ExprLit {
                attrs,
                lit: Lit::Bool(lit_bool),
            }) if attrs.is_empty() => {
                lit_bool.value().escape_and_write(self.buf);
            }
            _ => {
                self.values.push((self.buf.len(), expr));
            }
        }
    }

    pub fn write_attr(&mut self, attr: Attr) {
        let name = attr.name.to_string();
        if attr.is_optional() {
            let sep_name = String::from(' ') + &name;
            let sep_name_eq = sep_name.clone() + "=\"";

            match attr.value {
                AttrValue::Expr(value) => self.write_expr(parse_quote! {
                    ::core::option::Option::map(
                        #value,
                        |val| (::vy::PreEscaped(#sep_name_eq), val, vy::PreEscaped('"'))
                    )
                }),
                AttrValue::Bool(value) => self.write_expr(parse_quote!{
                    <bool>::then_some(#value, ::vy::PreEscaped(#sep_name))
                }),
            }
        } else {
            self.buf.push(' ');
            self.buf.push_str(&name);
            self.buf.push('=');
            self.buf.push('"');
            self.write_expr(attr.value.into());
            self.buf.push('"');
        }
    }

    pub fn write_element(&mut self, Element(head, body): Element) {
        let name = head.name.to_string();
        self.imports.push(head.name);
        self.buf.push('<');
        self.buf.push_str(&name);
        for attr in body.attrs {
            self.write_attr(attr);
        }
        self.buf.push('>');
        if !is_void_tag(&name) {
            for node in body.nodes {
                self.write_node(node);
            }
            self.buf.push('<');
            self.buf.push('/');
            self.buf.push_str(&name);
            self.buf.push('>');
        }
    }

    pub fn write_node(&mut self, node: Node) {
        match node {
            Node::Element(el) => self.write_element(el),
            Node::Expr(expr) => self.write_expr(expr),
        }
    }

    pub fn as_imports(&self) -> TokenStream {
        let imports = &self.imports;
        quote! {
            const _: () = {
                #(_ = #imports!(__vy_import_marker);)*
            }
        }
    }

    pub fn into_parts(self) -> Vec<Part<'s>> {
        let mut parts = Vec::new();
        let mut cursor = 0;

        for (i, val) in self.values {
            assert!(i >= cursor);
            let slice = &self.buf.as_str()[cursor..i];
            if !slice.is_empty() {
                parts.push(Part::Str(slice));
            }
            parts.push(Part::Expr(val));
            cursor = i;
        }

        let slice = &self.buf.as_str()[cursor..];
        if !slice.is_empty() {
            parts.push(Part::Str(slice));
        }

        parts
    }
}

pub enum Part<'s> {
    Str(&'s str),
    Expr(Expr),
}

const fn num_to_variant(n: u8) -> Option<char> {
    if n >= 1 && n <= 9 {
        Some((b'A' + n - 1) as char)
    } else {
        None
    }
}

fn wrap_branch(either: &Ident, count: u8, tokens: impl ToTokens) -> Block {
    let variant_letter =
        num_to_variant(count).expect("exceeded number of supported branches");
    let variant = format_ident!("{}", variant_letter);

    parse_quote!({
        ::vy::#either::#variant(#tokens)
    })
}

fn transform_branches(either: &Ident, mut count: u8, expr: &mut Expr) {
    count += 1;
    match expr {
        Expr::Block(ExprBlock { block, .. }) => {
            *block = wrap_branch(either, count, &block);
        }
        Expr::If(expr_if) => {
            expr_if.then_branch =
                wrap_branch(either, count, &expr_if.then_branch);

            if let Some((_, else_branch)) = &mut expr_if.else_branch {
                transform_branches(either, count, else_branch);
            } else {
                let unit: Expr = parse_quote!(());
                // Create a new `else` tail branch
                expr_if.else_branch = Some((
                    Token![else](Span::call_site()),
                    Box::new(Expr::Block(ExprBlock {
                        attrs: vec![],
                        label: None,
                        block: wrap_branch(either, count + 1, unit),
                    })),
                ))
            }
        }
        _ => {}
    }
}
