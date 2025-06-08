use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr, Result, Token,
};

use crate::{
    ast::ElementBody,
    generate::{Generator, Part},
};

mod ast;
mod generate;

#[cfg(feature = "speculative")]
mod speculative;

struct DefineElement {
    name: String,
    body: ElementBody,
}

impl Parse for DefineElement {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<LitStr>()?;
        _ = input.parse::<Token![,]>();

        Ok(Self {
            name: name.value(),
            body: ElementBody::parse(input)?,
        })
    }
}

fn compile_checks(el: &ElementBody) -> TokenStream2 {
    let mut items = Vec::new();

    fn recurse(items: &mut Vec<TokenStream2>, el: &ElementBody) {
        for attr in &el.attrs {
            let attr_name = &attr.name;
            items.push(quote! {
                _ = ::vy::known::Div::#attr_name;
            });
        }

        for node in &el.nodes {
            #[cfg(feature = "speculative")]
            if let ast::Node::Element(el) = node {
                let mac_path = &el.mac.path;
                items.push(quote! {
                    use #mac_path as _;
                });

                recurse(items, &el.body)
            }
        }
    }

    recurse(&mut items, el);

    quote!({
        #[allow(unused_imports)]
        use ::vy::known::AriaAttributes as _;
        #[allow(unused_imports)]
        use ::vy::known::GlobalAttributes as _;
        #[allow(unused_imports)]
        const _: () = {
            #(#items)*
        };
    })
}

/// Defines an element.
#[proc_macro]
pub fn define_element(input: TokenStream) -> TokenStream {
    let el = parse_macro_input!(input as DefineElement);
    let compile_checks = compile_checks(&el.body);

    let mut text = String::new();
    let mut gen = Generator::new(&mut text);
    gen.write_element(&el.name.to_string(), el.body);

    let parts = gen.finish().into_iter().map(|part| match part {
        Part::Text(text) => quote!(::vy::PreEscaped(#text)),
        Part::Expr(expr) => quote!(::vy::IntoHtml::into_html(#expr)),
    });

    quote!({
        #compile_checks

        (#(#parts),*)
    })
    .into()
}

/// Defines a void element.
///
/// # Examples
///
/// ```
/// declare_void_element!("input", id = 1 + 2);
/// ("<input id=\"", 1 + 2, "\">")
/// ```
///
/// ## Nested elements
///
/// ```
/// declare_element!("div", declare_void_element!("input", id = 1 + 2));
/// ("<div><input id=\"", 1 + 2, "\"></div>")
/// ```
#[proc_macro]
pub fn define_void_element(input: TokenStream) -> TokenStream {
    let element = parse_macro_input!(input with ElementBody::parse_void);

    quote! {}.into()
}
