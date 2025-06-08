#![allow(clippy::to_string_trait_impl)]

use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Expr, ExprMacro, Ident, LitStr, Result, Token,
};

/// An identifier: `foo`, or a literal: `"bar"`.
pub enum IdentOrLit {
    Ident(Ident),
    Lit(LitStr),
}

impl ToString for IdentOrLit {
    fn to_string(&self) -> String {
        match self {
            IdentOrLit::Ident(ident) => ident.to_string(),
            IdentOrLit::Lit(lit_str) => lit_str.value(),
        }
    }
}

impl Parse for IdentOrLit {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) {
            Ok(Self::Ident(input.parse()?))
        } else {
            Ok(Self::Lit(input.parse()?))
        }
    }
}

impl ToTokens for IdentOrLit {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Lit(lit_str) => lit_str.to_tokens(tokens),
        }
    }
}

pub enum Value {
    Expr(Expr),
}

impl Parse for Value {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self::Expr(input.parse()?))
    }
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Value::Expr(expr) => expr.to_tokens(tokens),
        }
    }
}

/// An attribute consisting of a key, value pair: `foo = "bar"`.
///
/// The key may consist of an identifier: `foo = "bar"`, or a literal:
/// `"bar" = "baz"`.
pub struct Attribute {
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.name.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

/// An attribute: `class = "foo"`, or a node: `1 + 2`.
pub enum AttrOrNode {
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

pub enum Node {
    Value(Value),
    #[cfg(feature = "speculative")]
    Element(crate::speculative::ElementMacro),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = input.parse()?;

        #[cfg(feature = "speculative")]
        if let Value::Expr(Expr::Macro(ExprMacro { mac, .. })) = &expr {
            if let Ok(el) = crate::speculative::ElementMacro::parse_macro(
                mac.clone(),
                input,
            ) {
                return Ok(Self::Element(el));
            }
        }

        Ok(Self::Value(expr))
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Node::Value(value) => value.to_tokens(tokens),
            #[cfg(feature = "speculative")]
            Node::Element(element) => element.to_tokens(tokens),
        }
    }
}

pub struct ElementBody {
    pub void: bool,
    pub attrs: Vec<Attribute>,
    pub nodes: Vec<Node>,
}

impl ElementBody {
    pub fn parse(input: ParseStream) -> Result<Self> {
        // The rest of the arguments are either attributes or nodes.
        //
        // Even though attributes are only allowed at the beginning, we parse
        // both attributes and nodes in any order to improve errors for
        // out-of-order attributes.
        let attrs_and_nodes =
            Punctuated::<AttrOrNode, Token![,]>::parse_terminated(input)?;

        let mut attrs = Vec::new();
        let mut nodes = Vec::new();

        for attr_or_node in attrs_and_nodes {
            match attr_or_node {
                AttrOrNode::Attr(attr) => {
                    if !nodes.is_empty() {
                        return Err(Error::new_spanned(
                            attr,
                            "attributes must be at the beginning",
                        ));
                    }

                    attrs.push(attr);
                }
                AttrOrNode::Node(expr) => {
                    nodes.push(expr);
                }
            }
        }

        Ok(Self {
            attrs,
            nodes,
            void: false,
        })
    }

    pub fn parse_void(input: ParseStream) -> Result<Self> {
        let el = ElementBody::parse(input)?;
        if !el.nodes.is_empty() {
            return Err(Error::new_spanned(
                el.nodes.first(),
                "void tags cannot contain content",
            ));
        }
        Ok(ElementBody { void: true, ..el })
    }
}
