use quote::ToTokens;
use syn::{Error, Expr, Ident, LitStr, Path, Result, Token};

pub struct Value(pub Expr);

/// `NodeName` captures all ways to define tag or attribute names and represents
/// them as faithfully as possible in the rendered output.
///
/// ```ignore
/// assert_eq!(
///     NodeName::Ident(parse_quote!(my_tag_name)).to_string(),
///     "my-tag-name"
/// );
/// assert_eq!(
///     NodeName::LitStr(parse_quote!("MY-weird_tag::name")).to_string(),
///     "MY-weird_tag::name"
/// );
/// assert_eq!(
///     NodeName::Path(parse_quote!(crate::custom_tag)).to_string(),
///     "custom-tag"
/// );
/// ```
pub enum NodeName {
    Ident(Ident),
    LitStr(LitStr),
    Path(Path),
}

// ..[]
pub struct SpreadAttr {
    pub dot_dot_token: Token![..],
    pub value: Value,
}

// foo = "bar"
pub struct KeyedAttr {
    pub name: NodeName,
    pub question_token: Option<Token![?]>,
    pub eq_token: Token![=],
    pub value: Value,
}

// ..[] or foo = "bar"
pub enum Attr {
    Spread(SpreadAttr),
    Keyed(KeyedAttr),
}

pub enum Node {
    CustomTagMacro(CustomTagMacro),
    CustomVoidTagMacro(CustomVoidTagMacro),
    KnownTagMacro(KnownTagMacro),
    Value(Value),
}

pub enum AttrOrNode {
    Attr(Attr),
    Node(Node),
}

#[derive(Default)]
pub struct ElementBody {
    pub attrs: Vec<Attr>,
    pub nodes: Vec<Node>,
}

pub struct Element {
    pub name: NodeName,
    pub void: bool,
    pub body: ElementBody,
}

// _tag!("tag-name", /* element body */ )
pub struct CustomTagMacro {
    pub path: Path,
    pub element: Element,
}

// _void_tag!("tag-name", /* element body */ )
pub struct CustomVoidTagMacro {
    pub path: Path,
    pub element: Element,
}

// div!(/* element body */ )
pub struct KnownTagMacro(pub Element);

impl From<Ident> for NodeName {
    fn from(value: Ident) -> Self {
        Self::Ident(value)
    }
}

impl From<LitStr> for NodeName {
    fn from(value: LitStr) -> Self {
        Self::LitStr(value)
    }
}

impl From<Path> for NodeName {
    fn from(value: Path) -> Self {
        Self::Path(value)
    }
}

impl From<CustomTagMacro> for Element {
    fn from(value: CustomTagMacro) -> Self {
        value.element
    }
}

impl From<KnownTagMacro> for Element {
    fn from(value: KnownTagMacro) -> Self {
        value.0
    }
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl ToTokens for NodeName {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::LitStr(lit_str) => lit_str.to_tokens(tokens),
            Self::Path(path) => path.to_tokens(tokens),
        }
    }
}

impl ToTokens for SpreadAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.dot_dot_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

impl ToTokens for KeyedAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.name.to_tokens(tokens);
        self.question_token.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

impl ToTokens for Attr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Attr::Spread(spread_attr) => spread_attr.to_tokens(tokens),
            Attr::Keyed(keyed_attr) => keyed_attr.to_tokens(tokens),
        }
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Node::CustomTagMacro(custom_tag_macro) => {
                custom_tag_macro.to_tokens(tokens)
            }
            Node::CustomVoidTagMacro(custom_void_tag_macro) => {
                custom_void_tag_macro.to_tokens(tokens)
            }
            Node::KnownTagMacro(known_tag_macro) => {
                known_tag_macro.to_tokens(tokens)
            }
            Node::Value(expr) => expr.to_tokens(tokens),
        }
    }
}

impl ToTokens for CustomTagMacro {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.path.to_tokens(tokens);
        self.element.to_tokens(tokens);
    }
}

impl ToTokens for CustomVoidTagMacro {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.path.to_tokens(tokens);
        self.element.to_tokens(tokens);
    }
}

impl ToTokens for KnownTagMacro {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl ToTokens for ElementBody {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        for node in &self.nodes {
            node.to_tokens(tokens);
        }
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.name.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

impl Element {
    pub fn new(name: NodeName, void: bool, body: ElementBody) -> Result<Self> {
        if void && !body.nodes.is_empty() {
            return Err(Error::new_spanned(
                body.nodes.first(),
                "void tags cannnot contain children",
            ));
        }

        Ok(Self { name, void, body })
    }
}
