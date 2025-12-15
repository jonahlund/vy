use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::ast;

pub trait ExpandDiagnostic {
    fn expand_diagnostic_to(&self, output: &mut Vec<TokenStream>);
}

impl ExpandDiagnostic for ast::Element {
    fn expand_diagnostic_to(&self, output: &mut Vec<TokenStream>) {
        if let ast::NodeName::Path(path) = &self.name {
            output.push(quote!(#path!(@diagnostic)));

            // theres probably a better way to do this..
            let erased_span_path: syn::Path =
                syn::parse_str(&path.to_token_stream().to_string()).unwrap();

            for attr in &self.body.attrs {
                if let ast::Attr::Keyed(ast::KeyedAttr {
                    name,
                    question_token,
                    eq_token,
                    value,
                }) = attr
                {
                    output.push(quote!(
                        #erased_span_path!(@diagnostic #name #question_token #eq_token #value)
                    ));
                }
            }
        }

        for node in &self.body.nodes {
            node.expand_diagnostic_to(output);
        }
    }
}

impl ExpandDiagnostic for ast::CustomTagMacro {
    fn expand_diagnostic_to(&self, output: &mut Vec<TokenStream>) {
        let Self { path, element } = self;
        output.push(quote!(#path!(@diagnostic)));
        element.expand_diagnostic_to(output);
    }
}

impl ExpandDiagnostic for ast::CustomVoidTagMacro {
    fn expand_diagnostic_to(&self, output: &mut Vec<TokenStream>) {
        let Self { path, element } = self;
        output.push(quote!(#path!(@diagnostic)));
        element.expand_diagnostic_to(output);
    }
}

impl ExpandDiagnostic for ast::KnownTagMacro {
    fn expand_diagnostic_to(&self, output: &mut Vec<TokenStream>) {
        self.0.expand_diagnostic_to(output);
    }
}

impl ExpandDiagnostic for ast::Node {
    fn expand_diagnostic_to(&self, output: &mut Vec<TokenStream>) {
        match self {
            Self::CustomTagMacro(custom_tag_macro) => {
                custom_tag_macro.expand_diagnostic_to(output)
            }
            Self::CustomVoidTagMacro(custom_void_tag_macro) => {
                custom_void_tag_macro.expand_diagnostic_to(output)
            }
            Self::KnownTagMacro(known_tag_macro) => {
                known_tag_macro.expand_diagnostic_to(output)
            }
            _ => {}
        }
    }
}
