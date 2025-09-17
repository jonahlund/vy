mod ast;
mod fmt;
#[macro_use]
mod known;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, parse_macro_input, Data, DeriveInput, Fields};
use vy_core::Buffer;

use self::{
    ast::{Element, ElementBody, ElementHead},
    fmt::{Part, Serializer},
};

mod kw {
    syn::custom_keyword!(__vy_import_marker);
}

enum Inner {
    Marker,
    Body(ElementBody),
}

impl Parse for Inner {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.parse::<kw::__vy_import_marker>().is_ok() {
            return Ok(Self::Marker);
        }

        Ok(Self::Body(input.parse()?))
    }
}

fn inner(name: &str, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as Inner);

    let body = match parsed {
        Inner::Marker => {
            return quote!(()).into();
        }
        Inner::Body(element_body) => element_body,
    };

    let el = Element::new(
        ElementHead {
            name: format_ident!("{}", name),
        },
        body,
    );

    let el = match el {
        Ok(el) => el,
        Err(err) => return err.to_compile_error().into(),
    };

    let mut text = Buffer::new();
    let mut ser = Serializer::new(&mut text);
    ser.write_element(el);

    let imports = ser.as_imports();
    let parts = ser.into_parts().into_iter().map(|part| match part {
        Part::Str(s) => quote!(::vy::PreEscaped(#s)),
        Part::Expr(e) => quote!(::vy::IntoHtml::into_html(#e)),
    });

    quote!({
        #imports;
        ( #(#parts),* )
    })
    .into()
}

macro_rules! define_proc_macro {
    ($($(#[doc=$doc:literal])* $el:ident)+) => {
        $(
            $(#[doc = $doc])*
            #[proc_macro]
            pub fn $el(input: TokenStream) -> TokenStream {
                inner(stringify!($el), input)
            }
        )+
    };
}

for_all_elements!(define_proc_macro);

/// Derives `IntoHtml` for a newtype struct.
///
/// This macro implements `IntoHtml` for a struct with a single tuple field
/// by delegating the implementation to the inner type.
///
/// # Example
///
/// ```
/// use vy_core::IntoHtml;
/// use vy_macros::InnerIntoHtml;
///
/// #[derive(InnerIntoHtml)]
/// struct Length((usize, String));
///
/// let length = Length((1, "cm".into()));
/// assert_eq!(length.into_string(), "1cm");
/// ```
#[proc_macro_derive(InnerIntoHtml)]
pub fn inner_into_html_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let data = match input.data {
        Data::Struct(s) => s,
        _ => panic!("InnerIntoHtml can only be derived for structs"),
    };

    let fields = match data.fields {
        Fields::Unnamed(f) => f.unnamed,
        _ => panic!("InnerIntoHtml can only be derived for tuple structs"),
    };

    if fields.len() != 1 {
        panic!(
            "InnerIntoHtml can only be derived for newtypes (tuple structs \
             with one field)"
        );
    }

    let expanded = quote! {
        impl ::vy_core::IntoHtml for #name {
            #[inline]
            fn into_html(self) -> impl ::vy_core::IntoHtml {
                self.0
            }
        }
    };

    TokenStream::from(expanded)
}
