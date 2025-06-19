use syn::{Expr, ExprGroup, ExprLit, Lit};
use vy_core::IntoHtml as _;

use super::{ElementBody, Node, Value};

pub struct Generator<'s> {
    pub output: &'s mut String,
    pub values: Vec<(usize, Expr)>,
}

impl<'s> Generator<'s> {
    pub fn new(output: &'s mut String) -> Self {
        Self {
            output,
            values: vec![],
        }
    }

    pub fn write_expr(&mut self, e: Expr) {
        match e {
            Expr::Group(ExprGroup { attrs, expr, .. }) if attrs.is_empty() => {
                self.write_expr(*expr);
            }
            Expr::Match(expr_match) => {
                // let mut matches = vec![];
            }
            Expr::If(mut expr_if) => {
                let count = 0;
            }
            Expr::Lit(ExprLit {
                attrs,
                lit: Lit::Str(lit_str),
            }) if attrs.is_empty() => {
                lit_str.value().escape_and_write(self.output);
            }
            Expr::Lit(ExprLit {
                attrs,
                lit: Lit::Char(lit_char),
            }) if attrs.is_empty() => {
                lit_char.value().escape_and_write(self.output);
            }
            Expr::Lit(ExprLit {
                attrs,
                lit: Lit::Int(lit_int),
            }) if attrs.is_empty() => {
                lit_int.base10_digits().escape_and_write(self.output);
            }
            Expr::Lit(ExprLit {
                attrs,
                lit: Lit::Float(lit_float),
            }) if attrs.is_empty() => {
                lit_float.base10_digits().escape_and_write(self.output);
            }
            Expr::Lit(ExprLit {
                attrs,
                lit: Lit::Bool(lit_bool),
            }) if attrs.is_empty() => {
                lit_bool.value().escape_and_write(self.output);
            }
            _ => {
                self.values.push((self.output.len(), e));
            }
        }
    }

    pub fn write_value(&mut self, v: Value) {
        match v {
            Value::Expr(expr) => {
                self.write_expr(expr);
            }
        }
    }

    pub fn write_node(&mut self, n: Node) {
        match n {
            Node::Value(value) => self.write_value(value),
            Node::Element(element) => {
                let name =
                    element.mac.path.segments.last().unwrap().ident.to_string();

                self.write_element(&name, element.body)
            }
        }
    }

    pub fn write_element(&mut self, name: &str, el: ElementBody) {
        let normalized_name = normalize_node_name(name);
        self.output.push('<');
        self.output.push_str(&normalized_name);
        for attr in el.attrs {
            self.output.push(' ');
            self.output
                .push_str(&normalize_node_name(&attr.name.to_string()));
            self.output.push('=');
            self.output.push('"');
            self.write_expr(attr.value);
            self.output.push('"');
        }
        self.output.push('>');
        if el.void {
            assert!(el.nodes.is_empty());
        } else {
            for node in el.nodes {
                self.write_node(node);
            }
            self.output.push('<');
            self.output.push('/');
            self.output.push_str(&normalized_name);
            self.output.push('>');
        }
    }

    pub fn parts(mut self) -> Vec<Part<'s>> {
        self.values.sort_by_key(|(i, _)| *i);

        let mut parts = Vec::new();
        let mut n = 0;

        for (i, expr) in self.values {
            let slice = &self.output[n..i];
            if !slice.is_empty() {
                parts.push(Part::Text(slice));
            }
            parts.push(Part::Expr(expr));
            n = i;
        }

        if n < self.output.len() {
            parts.push(Part::Text(&self.output[n..]));
        }

        parts
    }
}

pub enum Part<'s> {
    Text(&'s str),
    Expr(Expr),
}

fn normalize_node_name(name: &str) -> String {
    name.replace('_', "-").replace("r#", "")
}
