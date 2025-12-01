use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    Error, Expr, Ident, LitStr, Macro, Result, Token,
};

pub enum IdentOrLit {
    Ident(Ident),
    Lit(LitStr),
}

impl IdentOrLit {
    fn to_node_name(&self) -> String {
        match self {
            IdentOrLit::Ident(ident) => ident
                .to_string()
                .chars()
                .map(|ch| match ch {
                    '_' => '-',
                    _ => ch,
                })
                .collect(),
            IdentOrLit::Lit(lit_str) => lit_str.value(),
        }
    }
}

impl Parse for IdentOrLit {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            Ok(Self::Ident(input.parse()?))
        } else if lookahead.peek(LitStr) {
            Ok(Self::Lit(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct Attribute {
    pub name: IdentOrLit,
    pub eq_token: Token![=],
    pub value: Value,
}

pub enum Node {
    #[cfg(feature = "well-known")]
    KnownElement(KnownElement),
    Value(Value),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        #[cfg(feature = "well-known")]
        {
            let fork = input.fork();

            if let Ok(known_element) = fork.parse::<KnownElement>() {
                input.advance_to(&fork);
                return Ok(Self::KnownElement(known_element));
            }
        }

        Ok(Self::Value(input.parse()?))
    }
}

pub enum Value {
    Expr(Expr),
    Lit(LitStr),
}

impl Parse for Value {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Lit(input.parse()?))
        } else {
            Ok(Self::Expr(input.parse()?))
        }
    }
}

#[cfg(feature = "well-known")]
pub struct KnownElement {
    pub name: String,
    pub is_void: bool,
    pub mac: Macro,
}

#[cfg(feature = "well-known")]
impl Parse for KnownElement {
    fn parse(input: ParseStream) -> Result<Self> {
        let mac = input.parse::<Macro>()?;

        let name = mac
            .path
            .segments
            .last()
            .map(|seg| seg.ident.to_string())
            .unwrap_or_default();

        if !crate::well_known::is_known_tag(&name) {
            return Err(Error::new_spanned(mac, "not a known tag"));
        }

        let is_void = crate::well_known::is_known_void_tag(&name);

        Ok(Self { name, is_void, mac })
    }
}
