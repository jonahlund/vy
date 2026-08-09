mod known;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Expr, ExprMacro, Ident, LitStr, Macro, Result, Token,
};

fn ident_node_name(i: &Ident) -> String {
    node_name(&i.to_string())
}

fn node_name(s: &str) -> String {
    s.replace('_', "-")
}

enum IdentOrLit {
    Ident(Ident),
    Lit(LitStr),
}

impl Parse for IdentOrLit {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            Ok(Self::Lit(input.parse()?))
        } else if lookahead.peek(Ident) {
            Ok(Self::Ident(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToString for IdentOrLit {
    fn to_string(&self) -> String {
        match self {
            IdentOrLit::Ident(ident) => ident.to_string(),
            IdentOrLit::Lit(lit_str) => lit_str.value(),
        }
    }
}

impl ToTokens for IdentOrLit {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Lit(lit_str) => lit_str.to_tokens(tokens),
        }
    }
}

struct Attribute {
    pub name: IdentOrLit,
    pub eq_token: Token![=],
    pub value: Expr,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let eq_token = input.parse()?;
        let value = input.parse()?;

        Ok(Self {
            name,
            eq_token,
            value,
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.name.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

enum AttrOrNode {
    Attr(Attribute),
    Node(Node),
}

impl Parse for AttrOrNode {
    fn parse(input: ParseStream) -> Result<Self> {
        if (input.peek(Ident) || input.peek(LitStr))
            && ((input.peek2(Token![=]))
                || (input.peek2(Token![?]) && input.peek3(Token![=])))
        {
            Ok(Self::Attr(input.parse()?))
        } else {
            Ok(Self::Node(input.parse()?))
        }
    }
}

enum Node {
    Element(Element),
    Expr(Expr),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = input.parse()?;

        if let Expr::Macro(ExprMacro { mac, .. }) = &expr
            && let Some(ident) = mac.path.get_ident()
        {
            let name = ident_node_name(ident);
            if known::is_element(&name) {
                let is_void = known::is_void_element(&name);
                return Ok(Self::Element(Element::parse_known_macro(
                    name, is_void, mac,
                )?));
            }
        }

        Ok(Self::Expr(expr))
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Node::Expr(value) => value.to_tokens(tokens),
            Node::Element(element) => {
                // TODO
            }
        }
    }
}

struct Element {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub children: Vec<Node>,
    pub void: bool,
}

impl Element {
    fn parse_known_macro(
        name: String,
        void: bool,
        mac: &Macro,
    ) -> Result<Self> {
        let mut attrs = Vec::new();
        let mut children = Vec::new();

        for attr_or_node in mac.parse_body_with(
            Punctuated::<AttrOrNode, Token![,]>::parse_terminated,
        )? {
            match attr_or_node {
                AttrOrNode::Attr(attr) => {
                    if !children.is_empty() {
                        return Err(Error::new_spanned(
                            attr,
                            "attributes must be at the beginning",
                        ));
                    }

                    attrs.push(attr);
                }
                AttrOrNode::Node(node) => {
                    if void {
                        return Err(Error::new_spanned(
                            node,
                            "void element cannot contain children",
                        ));
                    }

                    children.push(node);
                }
            }
        }

        Ok(Self {
            name,
            attrs,
            children,
            void,
        })
    }
}

mod output {
    use syn::{Expr, ExprGroup, ExprLit, Lit};
    use vy_core::ToHtml as _;

    use crate::{node_name, Element, Node};

    fn format_expr<'e>(
        expr: &'e Expr,
        output_html: &mut String,
        output_args: &mut Vec<(usize, &'e Expr)>,
    ) {
        match expr {
            Expr::Group(ExprGroup { attrs, expr, .. }) if attrs.is_empty() => {
                format_expr(expr, output_html, output_args);
            }
            Expr::Lit(ExprLit { attrs, lit }) if attrs.is_empty() => {
                match lit {
                    Lit::Str(lit_str) => {
                        lit_str.value().escape_and_write(output_html);
                    }
                    Lit::Char(lit_char) => {
                        lit_char.value().escape_and_write(output_html);
                    }
                    Lit::Int(lit_int) => {
                        lit_int.base10_digits().escape_and_write(output_html);
                    }
                    Lit::Float(lit_float) => {
                        lit_float.base10_digits().escape_and_write(output_html);
                    }
                    Lit::Bool(lit_bool) => {
                        lit_bool.value().escape_and_write(output_html);
                    }
                    _ => {
                        output_args.push((output_html.len(), expr));
                    }
                }
            }
            _ => {
                output_args.push((output_html.len(), expr));
            }
        }
    }

    fn format_element<'e>(
        el: &'e Element,
        output_html: &mut String,
        output_args: &mut Vec<(usize, &'e Expr)>,
    ) {
        output_html.push('<');
        output_html.push_str(&el.name);
        for attr in &el.attrs {
            output_html.push(' ');
            output_html.push_str(&node_name(&attr.name.to_string()));
            output_html.push('=');
            output_html.push('"');
            format_expr(&attr.value, output_html, output_args);
            output_html.push('"');
        }
        output_html.push('>');
        if !el.void {
            for child in &el.children {
                format_node(child, output_html, output_args);
            }
            output_html.push('<');
            output_html.push('/');
            output_html.push_str(&el.name);
            output_html.push('>');
        }
    }

    fn format_node<'e>(
        node: &'e Node,
        output_html: &mut String,
        output_args: &mut Vec<(usize, &'e Expr)>,
    ) {
        match node {
            Node::Expr(expr) => format_expr(expr, output_html, output_args),
            Node::Element(el) => {
                format_element(el, output_html, output_args);
            }
        }
    }

    fn format(nodes: &[Node]) -> (String, Vec<(usize, &Expr)>) {
        let mut html_output = String::new();
        let mut args_output = Vec::new();

        for node in nodes {
            format_node(node, &mut html_output, &mut args_output);
        }

        (html_output, args_output)
    }
}

#[proc_macro]
pub fn vy(input: TokenStream) -> TokenStream {
    let nodes = syn::parse_macro_input!(input with Punctuated::<Token![,], Node>::parse_terminated);

    input
}
