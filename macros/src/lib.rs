mod fmt;

use fmt::{Part, Serializer};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Path, Result, Token,
};
use tiny_rsx::ast::Node;

struct Forward {
    to: Path,
    nodes: Vec<Node>,
}

impl Parse for Forward {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let to = input.parse()?;
        _ = input.parse::<Token![,]>()?;
        let nodes = parse_zero_or_more(input)?;

        Ok(Self { to, nodes })
    }
}

#[proc_macro]
pub fn forward(input: TokenStream) -> TokenStream {
    let Forward { to, nodes } = parse_macro_input!(input as Forward);

    let mut ser = Serializer::new();
    for node in nodes {
        ser.write_node(node);
    }
    let parts = ser.as_parts();

    quote! {
        #to!(#(#parts),*)
    }
    .into()
}

struct Closure {
    parts: Punctuated<Part<'static>, Token![,]>,
}

impl Parse for Closure {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            parts: Punctuated::parse_terminated(input)?,
        })
    }
}

#[proc_macro]
pub fn closure(input: TokenStream) -> TokenStream {
    let Closure { parts } = parse_macro_input!(input as Closure);

    let named_parts = parts
        .iter()
        .enumerate()
        .map(|(i, part)| (part, format_ident!("__arg{i}")))
        .collect::<Vec<_>>();
    let named_values = named_parts
        .iter()
        .filter_map(|(part, name)| {
            if let Part::Expr(val) = part {
                Some((val, name))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if named_values.is_empty() {
        return quote! {
            ::vy::PreEscaped(::std::concat!(#parts))
        }
        .into();
    }

    let stmts = named_parts.iter().map(|(part, name)| match part {
        Part::Str(s) => quote! {
            __buf.push_str(#s)
        },
        Part::Expr(_) => quote! {
            ::vy::IntoHtml::write_escaped(#name, __buf)
        },
    });
    let est_size = parts.iter().fold(0, |mut acc, part| {
        match part {
            Part::Str(s) => acc += s.len(),
            Part::Expr(expr) => {
                acc += {
                    let mut s = expr.to_token_stream().to_string();
                    s.retain(|ch| ch.is_alphanumeric());
                    s.len()
                }
            }
        };
        acc
    });
    let (values, names): (Vec<_>, Vec<_>) = named_values.into_iter().unzip();

    quote!(match (#(#values),*) {
        (#(#names),*) => {
            ::vy::from_fn(move |__buf| {
                __buf.reserve(#est_size);
                #(#stmts;)*
            })
        }
    })
    .into()
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Result<Vec<T>> {
    let mut res = Vec::new();
    while !input.is_empty() {
        res.push(input.parse()?);
    }
    Ok(res)
}
