#![allow(clippy::to_string_trait_impl)]

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Expr, ExprMacro, Ident, LitStr, Macro, Path, Result, Token,
};

use crate::known::{is_known_tag, is_void_tag};

/// A single ident `foo`, or a string literal `"bar"`.
pub enum AttrName {
    Ident(Ident),
    LitStr(LitStr),
}

impl Parse for AttrName {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            Ok(Self::Ident(input.parse()?))
        } else if lookahead.peek(LitStr) {
            Ok(Self::LitStr(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for AttrName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::LitStr(lit_str) => lit_str.to_tokens(tokens),
        }
    }
}

impl ToString for AttrName {
    fn to_string(&self) -> String {
        match self {
            Self::Ident(ident) => ident.to_string(),
            Self::LitStr(lit_str) => lit_str.value(),
        }
    }
}

pub struct Attr {
    pub name: AttrName,
    pub question_token: Option<Token![?]>,
    pub eq_token: Token![=],
    pub value: Expr,
}

impl Attr {
    pub const fn is_optional(&self) -> bool {
        self.question_token.is_some()
    }
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            name: input.parse()?,
            question_token: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl ToTokens for Attr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

pub struct ElementBody {
    pub attrs: Vec<Attr>,
    pub nodes: Vec<Node>,
}

impl Parse for ElementBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut parts =
            Punctuated::<AttrOrNode, Token![,]>::parse_terminated(input)?
                .into_iter()
                .peekable();

        let mut attrs = Vec::new();
        let mut nodes = Vec::new();

        while let Some(AttrOrNode::Attr(_)) = parts.peek() {
            let AttrOrNode::Attr(attr) = parts.next().unwrap() else {
                unreachable!();
            };
            attrs.push(attr);
        }

        for part in parts {
            match part {
                AttrOrNode::Attr(attr) => {
                    return Err(Error::new_spanned(
                        attr,
                        "attributes must be at the beginning",
                    ))
                }
                AttrOrNode::Node(node) => nodes.push(node),
            };
        }

        Ok(Self { attrs, nodes })
    }
}

impl ToTokens for ElementBody {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        for node in &self.nodes {
            node.to_tokens(tokens);
        }
    }
}

pub struct ElementHead {
    pub name: Ident,
}

impl ToTokens for ElementHead {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
    }
}

pub struct Element(pub ElementHead, pub ElementBody);

impl Element {
    pub fn new(head: ElementHead, body: ElementBody) -> Result<Self> {
        let name = head.name.to_string();

        if !is_known_tag(&name) {
            return Err(Error::new_spanned(name, "unknown tag name"));
        }

        if is_void_tag(&name) && !body.nodes.is_empty() {
            return Err(Error::new_spanned(
                body.nodes.first().unwrap(),
                "void tags cannot contain content",
            ));
        }

        Ok(Self(head, body))
    }

    pub fn from_macro(
        Macro {
            path: Path { mut segments, .. },
            tokens,
            ..
        }: Macro,
    ) -> Result<Self> {
        let name = segments.pop().unwrap().into_value().ident;

        let head = ElementHead { name };
        let body = syn::parse2(tokens)?;

        Self::new(head, body)
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
        self.1.to_tokens(tokens);
    }
}

pub enum Node {
    Element(Element),
    Expr(Expr),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = input.parse()?;
        if let Expr::Macro(ExprMacro { mac, .. }) = &expr {
            if let Ok(el) = Element::from_macro(mac.clone()) {
                return Ok(Self::Element(el));
            }
        }

        Ok(Self::Expr(expr))
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Node::Element(element) => element.to_tokens(tokens),
            Node::Expr(expr) => expr.to_tokens(tokens),
        }
    }
}

enum AttrOrNode {
    Attr(Attr),
    Node(Node),
}

impl Parse for AttrOrNode {
    fn parse(input: ParseStream) -> Result<Self> {
        if (input.peek(Ident) || input.peek(LitStr))
            && (input.peek2(Token![=])
                || (input.peek2(Token![?]) && input.peek3(Token![=])))
        {
            Ok(Self::Attr(input.parse()?))
        } else {
            Ok(Self::Node(input.parse()?))
        }
    }
}
