use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Ident, LitStr, Macro, Result, Token,
};

use crate::{
    ast,
    well_known::{is_known_tag, is_known_void_tag},
};

impl Parse for ast::Value {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl Parse for ast::NodeName {
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

impl Parse for ast::SpreadAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            dot_dot_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl Parse for ast::KeyedAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            name: input.parse()?,
            question_token: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl Parse for ast::Attr {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![..]) {
            Ok(Self::Spread(input.parse()?))
        } else {
            Ok(Self::Keyed(input.parse()?))
        }
    }
}

impl Parse for ast::Node {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();

        if let Ok(mac) = fork.parse::<Macro>() {
            let name = ast::NodeName::from(mac.path).to_string();

            if name == "-tag" {
                return Ok(Self::CustomTagMacro(input.parse()?));
            } else if name == "-void-tag" {
                return Ok(Self::CustomVoidTagMacro(input.parse()?));
            } else if is_known_tag(&name) {
                return Ok(Self::KnownTagMacro(input.parse()?));
            }
        }

        Ok(Self::Value(input.parse()?))
    }
}

impl Parse for ast::AttrOrNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let is_spread_attr = input.peek(Token![..]);
        let is_keyed_attr = (input.peek(Ident) || input.peek(LitStr))
            && ((input.peek2(Token![=]))
                || (input.peek2(Token![?]) && input.peek3(Token![=])));

        if is_spread_attr || is_keyed_attr {
            Ok(Self::Attr(input.parse()?))
        } else {
            Ok(Self::Node(input.parse()?))
        }
    }
}

impl Parse for ast::ElementBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs_and_nodes =
            Punctuated::<ast::AttrOrNode, Token![,]>::parse_terminated(input)?;
        let mut attrs = Vec::new();
        let mut nodes = Vec::new();

        for attr_or_node in attrs_and_nodes {
            match attr_or_node {
                ast::AttrOrNode::Attr(attr) => {
                    if !nodes.is_empty() {
                        return Err(Error::new_spanned(
                            attr,
                            "attributes must be at the beginning",
                        ));
                    }

                    attrs.push(attr);
                }
                ast::AttrOrNode::Node(expr) => {
                    nodes.push(expr);
                }
            }
        }

        Ok(Self { attrs, nodes })
    }
}

impl Parse for ast::CustomTagMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let mac = input.parse::<Macro>()?;

        let (name, body) =
            mac.parse_body_with(|input: &syn::parse::ParseBuffer| {
                let name = input.parse()?;
                let body = if input.parse::<Option<Token![,]>>()?.is_some() {
                    input.parse()?
                } else {
                    Default::default()
                };

                Ok((name, body))
            })?;

        Ok(Self {
            path: mac.path,
            element: ast::Element::new(name, false, body)?,
        })
    }
}

impl Parse for ast::CustomVoidTagMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let mac = input.parse::<Macro>()?;

        let (name, body) =
            mac.parse_body_with(|input: &syn::parse::ParseBuffer| {
                let name = input.parse()?;
                let body = if input.parse::<Option<Token![,]>>()?.is_some() {
                    input.parse()?
                } else {
                    Default::default()
                };

                Ok((name, body))
            })?;

        Ok(Self {
            path: mac.path,
            element: ast::Element::new(name, true, body)?,
        })
    }
}

impl Parse for ast::KnownTagMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let mac = input.parse::<Macro>()?;
        let name = ast::NodeName::Path(mac.path.clone());
        let name_str = name.to_string();

        if !is_known_tag(&name_str) {
            return Err(Error::new_spanned(
                mac.path.segments.last(),
                "not a known tag",
            ));
        };

        let void = is_known_void_tag(&name_str);

        Ok(Self(ast::Element::new(name, void, mac.parse_body()?)?))
    }
}
