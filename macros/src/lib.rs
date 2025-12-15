use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, Block, Ident, Result, Token,
};

use crate::{
    diagnostic::ExpandDiagnostic,
    fmt::{Expand, Part, PartBuffer},
};

mod ast;
mod diagnostic;
mod fmt;
mod parse;
mod well_known;

#[proc_macro]
pub fn _compile(input: TokenStream) -> TokenStream {
    let nodes = parse_macro_input!(input with Punctuated<ast::Node, Token![,]>::parse_terminated);

    let mut output_diagnostic = Vec::new();
    let mut output_text = String::new();
    let mut output_values = Vec::new();
    let mut output = PartBuffer::new(&mut output_text, &mut output_values);

    for node in &nodes {
        node.expand_to(&mut output);
        node.expand_diagnostic_to(&mut output_diagnostic);
    }

    let parts = output.to_parts().into_iter().map(|part| match part {
        Part::Text(string) => quote!(::vy::PreEscaped(#string)),
        Part::Expr(expr) => quote!(::vy::IntoHtml::into_html(#expr)),
    });

    quote!({
        const _: () = {
            #(#output_diagnostic;)*
        };

        ::vy::IntoHtml::into_html((#(#parts),*))
    })
    .into()
}

struct DefineHtml5Element {
    macro_attrs: Vec<Attribute>,
    macro_name: Ident,
    struct_name: Ident,
    struct_impl: Block,
}

impl Parse for DefineHtml5Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let macro_attrs = input.call(Attribute::parse_outer)?;
        let macro_name = input.parse()?;
        input.parse::<Token![impl]>()?;
        let struct_name = input.parse()?;
        let struct_impl = input.parse()?;

        Ok(Self {
            macro_attrs,
            macro_name,
            struct_name,
            struct_impl,
        })
    }
}

#[proc_macro]
pub fn _define_html5_elements(input: TokenStream) -> TokenStream {
    let elements = parse_macro_input!(input with Punctuated::<DefineHtml5Element, Token![,]>::parse_terminated);

    let definitions = elements.into_iter().map(|el| {
        let DefineHtml5Element {
            macro_attrs,
            macro_name,
            struct_name,
            struct_impl,
        } = el;

        quote! {
            pub struct #struct_name;

            #[macro_export]
            #(#macro_attrs)*
            macro_rules! #macro_name {
                (@diagnostic $key:ident $(?)? = $val:expr) => {{
                    #[allow(unused_imports)]
                    use ::vy::well_known::html5::{
                        GlobalAttributes as _,
                        AriaAttributes as _
                    };
                    _ = ::vy::well_known::html5::#struct_name::$key;
                }};
                (@diagnostic $($tt:tt)*) => {};
                ($($tt:tt)*) => {
                    ::vy::_compile!(::vy::#macro_name!($($tt)*))
                };
            }

            impl #struct_name #struct_impl

            impl Element for #struct_name {}
        }
    });

    quote! {
        #(#definitions)*
    }
    .into()
}
