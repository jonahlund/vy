use syn::{Expr, ExprGroup, ExprLit, Lit};
use tiny_rsx::ast::{Attr, Element, Node, Value, VoidTag};
use vy_runtime::ToHtml as _;

use crate::InsertAt;

pub struct Formatter<'a> {
    pub literal: String,
    pub values: Vec<InsertAt<'a>>,
}

impl<'a> Formatter<'a> {
    pub fn new() -> Self {
        Self {
            literal: String::new(),
            values: Vec::new(),
        }
    }

    pub fn write_expr(&mut self, expr: &'a Expr) {
        match &expr {
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
                self.write_expr(expr);
            }
            _ => {
                self.values.push(InsertAt(self.literal.len(), expr));
            }
        }
    }

    pub fn write_value(&mut self, val: &'a Value) {
        match val {
            Value::Expr(expr) => self.write_expr(expr),
            Value::LitStr(lit_str) => {
                lit_str.value().write_escaped(&mut self.literal);
            }
        }
    }

    pub fn write_attribute(&mut self, attr: &'a Attr) {
        self.literal.push(' ');
        self.literal.push_str(&attr.key.to_string());
        self.literal.push('=');
        self.literal.push('"');
        self.write_value(&attr.value);
        self.literal.push('"');
    }

    pub fn write_element(&mut self, el: &'a Element) {
        self.literal.push('<');
        match el {
            Element::OpeningClosing {
                opening_tag,
                children,
                ..
            } => {
                let name = opening_tag.name.to_string();
                self.literal.push_str(&name);
                for attr in &opening_tag.attrs {
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

    pub fn write_node(&mut self, node: &'a Node) {
        match node {
            Node::Value(value) => self.write_value(value),
            Node::Element(element) => self.write_element(element),
        }
    }
}
