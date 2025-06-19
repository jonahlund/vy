mod declare;
mod define;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote};

use crate::{
    declare::DeclareElements,
    define::{
        compile_checks,
        generate::{Generator, Part},
        DefineElement, DefineVoidElement,
    },
};

/// Defines an element.
#[proc_macro]
pub fn define_element(input: TokenStream) -> TokenStream {
    let el = parse_macro_input!(input as DefineElement);
    let el_name = format_ident!("{}", el.name.value());
    let compile_checks = compile_checks(&parse_quote!(#el_name), &el.body);

    let mut text = String::new();
    let mut gen = Generator::new(&mut text);
    gen.write_element(&el.name.value(), el.body);

    let parts = gen.parts().into_iter().map(|part| match part {
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
    let el = parse_macro_input!(input as DefineVoidElement);
    let el_name = format_ident!("{}", el.name.value());
    let compile_checks = compile_checks(&parse_quote!(#el_name), &el.body);

    let mut text = String::new();
    let mut gen = Generator::new(&mut text);
    gen.write_element(&el.name.value(), el.body);

    let parts = gen.parts().into_iter().map(|part| match part {
        Part::Text(text) => quote!(::vy::PreEscaped(#text)),
        Part::Expr(expr) => quote!(::vy::IntoHtml::into_html(#expr)),
    });

    quote!({
        #compile_checks

        (#(#parts),*)
    })
    .into()
}

#[proc_macro]
pub fn declare_elements(input: TokenStream) -> TokenStream {
    let DeclareElements { elements } =
        parse_macro_input!(input as DeclareElements);

    let elements = elements.into_iter().map(|el| {
        let el_name_ident = el.name;
        let el_name_str = el_name_ident.to_string();
        let el_attrs = el.attributes.into_iter().map(|attr| {
            let docs = attr.docs;
            let name = attr.name;

            quote!(
                #(#docs)*
                #[expect(non_upper_case_globals)]
                pub const #name: ::vy::Attribute = ::vy::Attribute;
            )
        });

        let docs = el.docs;

        quote! {
            #[expect(non_camel_case_types)]
            #[doc(hidden)]
            pub struct #el_name_ident;

            impl ::vy::Element for #el_name_ident {}

            impl #el_name_ident {
                #(#el_attrs)*
            }

            #(#docs)*
            #[macro_export]
            macro_rules! #el_name_ident {
                ($($tt:tt)*) => {
                    ::vy::define_element!(#el_name_str, $($tt)*)
                }
            }
        }
    });

    quote! {
        #(#elements)*
    }
    .into()
}
