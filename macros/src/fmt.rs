use std::borrow::Cow;

use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    Expr, ExprGroup, ExprLit, Lit, LitStr,
};
use tiny_rsx::ast::{Attr, Element, Node, Value, VoidTag};
use vy_core::ToHtml as _;

struct InsertAt(usize, Expr);

pub enum Part<'s> {
    Str(Cow<'s, str>),
    Expr(Cow<'s, Expr>),
}

impl Parse for Part<'_> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Str(input.parse::<LitStr>()?.value().into()))
        } else {
            Ok(Self::Expr(Cow::Owned(input.parse()?)))
        }
    }
}

impl ToTokens for Part<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Part::Str(str) => str.to_tokens(tokens),
            Part::Expr(expr) => expr.to_tokens(tokens),
        }
    }
}

pub struct Serializer {
    literal: String,
    values: Vec<InsertAt>,
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            literal: String::new(),
            values: Vec::new(),
        }
    }

    pub fn write_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => {
                lit_str.value().write_escaped(&mut self.literal);
            }
            Expr::Lit(ExprLit {
                lit: Lit::Bool(lit_bool),
                ..
            }) => {
                lit_bool.value.write_escaped(&mut self.literal);
            }
            Expr::Lit(ExprLit {
                lit: Lit::Char(lit_char),
                ..
            }) => {
                lit_char.value().write_escaped(&mut self.literal);
            }
            Expr::Lit(ExprLit {
                lit: Lit::Int(lit_int),
                ..
            }) => {
                lit_int.base10_digits().write_escaped(&mut self.literal);
            }
            Expr::Lit(ExprLit {
                lit: Lit::Float(lit_float),
                ..
            }) => {
                lit_float.base10_digits().write_escaped(&mut self.literal);
            }
            Expr::Group(ExprGroup { expr, .. }) => {
                self.write_expr(*expr);
            }
            _ => {
                self.values.push(InsertAt(self.literal.len(), expr));
            }
        }
    }

    pub fn write_value(&mut self, val: Value) {
        match val {
            Value::Expr(expr) => self.write_expr(expr),
            Value::LitStr(lit_str) => {
                lit_str.value().write_escaped(&mut self.literal);
            }
        }
    }

    pub fn write_attribute(&mut self, attr: Attr) {
        self.literal.push(' ');
        self.literal.push_str(&attr.key.to_string());
        self.literal.push('=');
        self.literal.push('"');
        self.write_value(attr.value);
        self.literal.push('"');
    }

    pub fn write_element(&mut self, el: Element) {
        self.literal.push('<');
        match el {
            Element::OpeningClosing {
                opening_tag,
                children,
                ..
            } => {
                let name = opening_tag.name.to_string();
                self.literal.push_str(&name);
                for attr in opening_tag.attrs {
                    self.write_attribute(attr);
                }
                self.literal.push('>');
                for child in children {
                    match child {
                        Node::Value(val) => {
                            self.write_value(val);
                        }
                        Node::Element(el) => {
                            self.write_element(el);
                        }
                    }
                }
                self.literal.push('<');
                self.literal.push('/');
                self.literal.push_str(&name);
                self.literal.push('>');
            }
            Element::Void(VoidTag { name, attrs, .. }) => {
                self.literal.push_str(&name.to_string());
                for attr in attrs {
                    self.write_attribute(attr);
                }
                self.literal.push('>');
            }
        }
    }

    pub fn write_node(&mut self, node: Node) {
        match node {
            Node::Value(value) => self.write_value(value),
            Node::Element(element) => self.write_element(element),
        }
    }

    pub fn as_parts(&self) -> Vec<Part> {
        let mut parts = Vec::new();
        let mut cursor = 0;

        for InsertAt(i, val) in &self.values {
            assert!(*i >= cursor);
            let slice = &self.literal[cursor..*i];
            if !slice.is_empty() {
                parts.push(Part::Str(slice.into()));
            }
            parts.push(Part::Expr(Cow::Borrowed(val)));
            cursor = *i;
        }

        let slice = &self.literal[cursor..];
        if !slice.is_empty() {
            parts.push(Part::Str(slice.into()));
        }

        parts
    }
}
