use core::fmt::{Result, Write};

use quote::ToTokens;
use syn::{Expr, ExprLit};
use vy_core::ToHtml;

use crate::ast;

pub struct Formatter<'a, 'b> {
    pub buf: &'a mut String,
    pub args: &'a mut Vec<(usize, &'b dyn ToTokens)>,
}

impl Write for Formatter<'_, '_> {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result {
        self.buf.write_str(s)
    }
}

impl<'a, 'b> Formatter<'a, 'b> {
    #[inline]
    pub fn new(
        buf: &'a mut String,
        args: &'a mut Vec<(usize, &'b dyn ToTokens)>,
    ) -> Self {
        Self { buf, args }
    }

    #[inline]
    pub fn write_ident(&mut self, i: &ast::DashIdent) {
        let _ = write!(self, "{}", i);
    }

    #[inline]
    pub fn write_doctype(&mut self, _: &ast::Doctype) {
        let _ = self.write_str("<!DOCTYPE html>");
    }

    pub fn write_expr(&mut self, e: &'b Expr) {
        if let Expr::Lit(ExprLit { attrs, lit }) = e {
            if attrs.is_empty() {
                match lit {
                    syn::Lit::Str(str) => {
                        return str.value().to_html(self.buf);
                    }
                    syn::Lit::Char(char) => {
                        return char.value().to_html(self.buf);
                    }
                    syn::Lit::Int(int) => {
                        return int.base10_digits().to_html(self.buf);
                    }
                    syn::Lit::Float(float) => {
                        return float.base10_digits().to_html(self.buf);
                    }
                    syn::Lit::Bool(bool) => {
                        return bool.value().to_html(self.buf);
                    }
                    _ => {}
                }
            }
        }
        self.args.push((self.buf.len(), e));
    }

    #[inline]
    pub fn write_value(&mut self, v: &'b ast::Value) {
        match v {
            ast::Value::LitStr(lit_str) => {
                lit_str.value().to_html(self.buf);
            }
            ast::Value::Expr(expr) => self.write_expr(expr),
        }
    }

    pub fn write_attr(&mut self, a: &'b ast::Attr) {
        self.write_ident(&a.key);
        let _ = self.write_char('=');
        let _ = self.write_char('"');
        self.write_value(&a.value);
        let _ = self.write_char('"');
    }

    pub fn write_tag(&mut self, t: &'b ast::Tag) {
        match t {
            ast::Tag::Opening { name, attrs, .. } => {
                let _ = self.write_char('<');
                self.write_ident(name);
                for attr in attrs {
                    let _ = self.write_char(' ');
                    self.write_attr(attr);
                }
                let _ = self.write_char('>');
            }
            ast::Tag::Closing { name, .. } => {
                let _ = self.write_char('<');
                let _ = self.write_char('/');
                self.write_ident(name);
                let _ = self.write_char('>');
            }
        }
    }

    #[inline]
    pub fn write_node(&mut self, n: &'b ast::Node) {
        match n {
            ast::Node::Doctype(d) => self.write_doctype(d),
            ast::Node::Tag(t) => self.write_tag(t),
            ast::Node::Value(v) => self.write_value(v),
        }
    }
}
