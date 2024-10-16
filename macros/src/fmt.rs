use core::fmt::{Result, Write};

use quote::ToTokens;
use syn::{Expr, ExprLit};
use vy_core::Render;

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
    pub fn write_ident(&mut self, i: &ast::DashIdent) -> Result {
        write!(self, "{}", i)
    }

    #[inline]
    pub fn write_doctype(&mut self, _: &ast::Doctype) -> Result {
        self.write_str("<!DOCTYPE html>")
    }

    pub fn write_expr(&mut self, e: &'b Expr) -> Result {
        if let Expr::Lit(ExprLit { attrs, lit }) = e {
            if attrs.is_empty() {
                match lit {
                    syn::Lit::Str(lit_str) => {
                        lit_str.value().render_to(self.buf);
                        return Ok(());
                    }
                    syn::Lit::Char(lit_char) => {
                        lit_char.value().render_to(self.buf);
                        return Ok(());
                    }
                    syn::Lit::Int(lit_int) => {
                        lit_int.base10_digits().render_to(self.buf);
                        return Ok(());
                    }
                    syn::Lit::Float(lit_float) => {
                        lit_float.base10_digits().render_to(self.buf);
                        return Ok(());
                    }
                    syn::Lit::Bool(lit_bool) => {
                        lit_bool.value().render_to(self.buf);
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
        self.args.push((self.buf.len(), e));
        Ok(())
    }

    #[inline]
    pub fn write_value(&mut self, v: &'b ast::Value) -> Result {
        match v {
            ast::Value::LitStr(lit_str) => {
                lit_str.value().render_to(self.buf);
                Ok(())
            }
            ast::Value::Expr(expr) => self.write_expr(expr),
        }
    }

    pub fn write_attr(&mut self, a: &'b ast::Attr) -> Result {
        self.write_ident(&a.key)?;
        self.write_char('=')?;
        self.write_char('"')?;
        self.write_value(&a.value)?;
        self.write_char('"')
    }

    pub fn write_tag(&mut self, t: &'b ast::Tag) -> Result {
        match t {
            ast::Tag::Opening { name, attrs, .. } => {
                self.write_char('<')?;
                self.write_ident(name)?;
                for attr in attrs {
                    self.write_char(' ')?;
                    self.write_attr(attr)?;
                }
                self.write_char('>')
            }
            ast::Tag::Closing { name, .. } => {
                self.write_char('<')?;
                self.write_char('/')?;
                self.write_ident(name)?;
                self.write_char('>')
            }
        }
    }

    #[inline]
    pub fn write_node(&mut self, n: &'b ast::Node) -> Result {
        match n {
            ast::Node::Doctype(d) => self.write_doctype(d),
            ast::Node::Tag(t) => self.write_tag(t),
            ast::Node::Value(v) => self.write_value(v),
        }
    }
}
