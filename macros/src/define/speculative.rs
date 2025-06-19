use quote::ToTokens;
use syn::{parse::ParseStream, Macro, Result};

use super::ElementBody;

#[rustfmt::skip]
const KNOWN_ELEMENTS: &[&str] = &[
    "a", "abbr", "address", "area", "article", "aside", "audio",
    "b", "base", "bdi", "bdo", "blockquote", "body", "br", "button",
    "canvas", "caption", "cite", "code", "col", "colgroup",
    "data", "datalist", "dd", "del", "details", "dfn", "dialog", "div", "dl", "dt",
    "em", "embed",
    "fieldset", "figcaption", "figure", "footer", "form",
    "h1", "h2", "h3", "h4", "h5", "h6", "head", "header", "hgroup", "hr", "html",
    "i", "iframe", "img", "input", "ins",
    "kbd",
    "label", "legend", "li", "link",
    "main", "map", "mark", "menu", "meta", "meter",
    "nav", "noscript",
    "object", "ol", "optgroup", "option", "output",
    "p", "picture", "pre", "progress",
    "q",
    "rp", "rt", "ruby",
    "s", "samp", "script", "search", "section", "select", "slot", "small", "source", "span", "strong", "style", "sub", "summary", "sup",
    "table", "tbody", "td", "template", "textarea", "tfoot", "th", "thead", "time", "title", "tr", "track",
    "u", "ul",
    "var", "video",
    "wbr"
];

fn is_known_element(name: &str) -> bool {
    // https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements

    KNOWN_ELEMENTS.binary_search(&name).is_ok()
}

fn is_void_element(name: &str) -> bool {
    // https://developer.mozilla.org/en-US/docs/Glossary/Void_element
    const KNOWN: &[&str] = &[
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link",
        "meta", "source", "track", "wbr",
    ];
    KNOWN.contains(&name)
}

pub struct ElementMacro {
    pub mac: Macro,
    pub body: ElementBody,
}

impl ElementMacro {
    pub fn parse_macro(mac: Macro, input: ParseStream) -> Result<Self> {
        let name = mac.path.segments.last().map(|s| s.ident.clone());

        let body = match name.as_ref().map(|s| s.to_string()) {
            Some(name) if is_void_element(&name) => {
                mac.parse_body_with(ElementBody::parse_void)?
            }
            Some(name) if is_known_element(&name) => {
                mac.parse_body_with(ElementBody::parse)?
            }
            _ => return Err(input.error("macro name is not a known element")),
        };

        Ok(Self { mac, body })
    }
}

impl ToTokens for ElementMacro {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.mac.to_tokens(tokens);
    }
}
