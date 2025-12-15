use std::{borrow::Cow, fmt::Display};

use quote::format_ident;
use syn::{parse_quote, Arm, Block, Expr, ExprIf, ExprLit, ExprMatch, Lit};
use vy_core::IntoHtml;

use crate::ast;

pub struct ExprIndice<'e> {
    pub index: usize,
    pub expr: Cow<'e, Expr>,
}

pub enum Part<'t, 'e> {
    Text(&'t str),
    Expr(&'e Expr),
}

pub struct PartBuffer<'o, 'e> {
    text: &'o mut String,
    values: &'o mut Vec<ExprIndice<'e>>,
}

impl<'o, 'e> PartBuffer<'o, 'e> {
    pub fn new(
        text: &'o mut String,
        values: &'o mut Vec<ExprIndice<'e>>,
    ) -> Self {
        Self { text, values }
    }

    fn push_owned_expr(&mut self, expr: Expr) {
        self.values.push(ExprIndice {
            index: self.text.len(),
            expr: Cow::Owned(expr),
        });
    }

    fn push_borrowed_expr(&mut self, expr: &'e Expr) {
        self.values.push(ExprIndice {
            index: self.text.len(),
            expr: Cow::Borrowed(expr),
        });
    }

    fn push_str(&mut self, string: &str) {
        self.text.push_str(string);
    }

    fn push_char(&mut self, ch: char) {
        self.text.push(ch);
    }

    pub fn to_parts(&'e self) -> Vec<Part<'o, 'e>> {
        let mut parts = Vec::new();
        let mut n = 0;

        for val in self.values.iter() {
            let slice = &self.text[n..val.index];
            if !slice.is_empty() {
                parts.push(Part::Text(slice));
            }
            parts.push(Part::Expr(&val.expr));
            n = val.index;
        }

        if n < self.text.len() {
            parts.push(Part::Text(&self.text[n..]));
        }

        parts
    }
}

pub trait Expand {
    fn expand_to<'e>(&'e self, output: &mut PartBuffer<'_, 'e>);
}

impl Display for ast::NodeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(ident) => {
                ident
                    .to_string()
                    .replace("r#", "")
                    .replace('_', "-")
                    .fmt(f)?;
            }
            Self::LitStr(lit_str) => {
                lit_str.value().fmt(f)?;
            }
            Self::Path(path) => {
                path.segments
                    .last()
                    .map(|seg| ast::NodeName::from(seg.ident.clone()).fmt(f));
            }
        };
        Ok(())
    }
}

impl Expand for ast::Value {
    fn expand_to<'e>(&'e self, output: &mut PartBuffer<'_, 'e>) {
        match &self.0 {
            Expr::If(_) | Expr::Match(_) => {
                let mut expr = self.0.clone();
                wrap_branches_in_either(&mut expr);
                output.push_owned_expr(expr);
                return;
            }
            Expr::Lit(ExprLit { lit, attrs }) if attrs.is_empty() => match &lit
            {
                Lit::Str(lit_str) => {
                    lit_str.value().escape_and_write(output.text);
                    return;
                }
                Lit::Char(lit_char) => {
                    lit_char.value().escape_and_write(output.text);
                    return;
                }
                Lit::Int(lit_int) => {
                    lit_int.base10_digits().escape_and_write(output.text);
                    return;
                }
                Lit::Float(lit_float) => {
                    lit_float.base10_digits().escape_and_write(output.text);
                    return;
                }
                Lit::Bool(lit_bool) => {
                    lit_bool.value().escape_and_write(output.text);
                    return;
                }
                _ => {}
            },
            _ => {}
        }

        output.push_borrowed_expr(&self.0);
    }
}

impl Expand for ast::SpreadAttr {
    fn expand_to<'e>(&'e self, output: &mut PartBuffer<'_, 'e>) {
        let value = &self.value;
        output.push_owned_expr(parse_quote! { ::vy::SpreadAttributes(#value) });
    }
}

impl Expand for ast::KeyedAttr {
    fn expand_to<'e>(&'e self, output: &mut PartBuffer<'_, 'e>) {
        if self.question_token.is_some() {
            let name = &self.name.to_string();
            let value = &self.value;
            output.push_owned_expr(
                parse_quote! { ::vy::OptionalAttribute(#name, #value) },
            );
        } else {
            output.push_char(' ');
            output.push_str(&self.name.to_string());
            output.push_char('=');
            output.push_char('"');
            self.value.expand_to(output);
            output.push_char('"');
        }
    }
}

impl Expand for ast::Attr {
    fn expand_to<'e>(&'e self, output: &mut PartBuffer<'_, 'e>) {
        match self {
            Self::Spread(spread_attr) => spread_attr.expand_to(output),
            Self::Keyed(keyed_attr) => keyed_attr.expand_to(output),
        }
    }
}

impl Expand for ast::Element {
    fn expand_to<'e>(&'e self, output: &mut PartBuffer<'_, 'e>) {
        let name = self.name.to_string();
        output.push_char('<');
        output.push_str(&name);

        for attr in &self.body.attrs {
            attr.expand_to(output);
        }

        output.push_char('>');

        if !self.void {
            for node in &self.body.nodes {
                node.expand_to(output);
            }

            output.push_str("</");
            output.push_str(&name);
            output.push_char('>');
        }
    }
}

impl Expand for ast::CustomTagMacro {
    fn expand_to<'e>(&'e self, output: &mut PartBuffer<'_, 'e>) {
        self.element.expand_to(output);
    }
}

impl Expand for ast::CustomVoidTagMacro {
    fn expand_to<'e>(&'e self, output: &mut PartBuffer<'_, 'e>) {
        self.element.expand_to(output);
    }
}

impl Expand for ast::KnownTagMacro {
    fn expand_to<'e>(&'e self, output: &mut PartBuffer<'_, 'e>) {
        self.0.expand_to(output);
    }
}

impl Expand for ast::Node {
    fn expand_to<'e>(&'e self, output: &mut PartBuffer<'_, 'e>) {
        match self {
            Self::CustomTagMacro(custom_tag_macro) => {
                custom_tag_macro.expand_to(output)
            }
            Self::CustomVoidTagMacro(custom_void_tag_macro) => {
                custom_void_tag_macro.expand_to(output)
            }
            Self::KnownTagMacro(known_tag_macro) => {
                known_tag_macro.expand_to(output)
            }
            Self::Value(value) => value.expand_to(output),
        }
    }
}

fn wrap_branches_in_either(expr: &mut Expr) {
    if let Expr::If(ExprIf { else_branch, .. }) = expr {
        else_branch
            .get_or_insert((Default::default(), Box::new(parse_quote!({}))));
    }

    if let Expr::Match(ExprMatch { arms, .. }) = expr {
        for Arm { body, .. } in arms {
            *body = parse_quote!({ #body });
        }
    }

    let branch_count = count_branches(expr);

    let either_ident = if branch_count <= 2 {
        format_ident!("Either")
    } else {
        format_ident!("Either{}", branch_count)
    };
    let mut either_variant = b'A';

    visit_branch_block(expr, &mut |block| {
        let either_variant_ident =
            format_ident!("{}", char::from_u32(either_variant as u32).unwrap());
        *block = parse_quote!({
            ::vy::#either_ident::#either_variant_ident(#block)
        });
        either_variant += 1;
    });
}

/// Counts the branches in the given expression
fn count_branches(e: &mut Expr) -> usize {
    let mut n = 0;
    visit_branch_block(e, &mut |_| {
        n += 1;
    });
    n
}

/// A branch visitor that may visit the following branch kinds:
/// - then branches
/// - else branches
/// - match branches
///
/// It calls the closure with a mutable reference to the branch block
fn visit_branch_block<F: FnMut(&mut Block)>(e: &mut Expr, f: &mut F) {
    match e {
        Expr::Block(expr_block) => f(&mut expr_block.block),
        Expr::If(ExprIf {
            then_branch,
            else_branch,
            ..
        }) => {
            f(then_branch);
            if let Some((_, else_branch)) = else_branch {
                visit_branch_block(else_branch, f);
            }
        }
        Expr::Match(ExprMatch { arms, .. }) => {
            for arm in arms {
                visit_branch_block(&mut arm.body, f);
            }
        }
        _ => {}
    }
}
