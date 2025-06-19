use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Result, Token,
};

pub struct Attribute {
    pub docs: Vec<syn::Attribute>,
    pub name: Ident,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            docs: input.call(syn::Attribute::parse_outer)?,
            name: input.parse()?,
        })
    }
}

pub struct Element {
    pub docs: Vec<syn::Attribute>,
    pub name: Ident,
    pub attributes: Punctuated<Attribute, Token![,]>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let docs = input.call(syn::Attribute::parse_outer)?;
        let name = input.parse()?;
        let attributes;
        braced!(attributes in input);

        Ok(Self {
            docs,
            name,
            attributes: attributes
                .parse_terminated(Attribute::parse, Token![,])?,
        })
    }
}

pub struct DeclareElements {
    pub elements: Punctuated<Element, Token![,]>,
}

impl Parse for DeclareElements {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            elements: input.parse_terminated(Element::parse, Token![,])?,
        })
    }
}
