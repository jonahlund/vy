use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use tiny_rsx::Formatter;

use crate::{LazyInput, WriteInput};

pub(crate) fn lazy(input: LazyInput) -> TokenStream {
    let mut text = String::new();
    let mut args = Vec::new();

    let mut formatter = Formatter::new(&mut text, &mut args);
    for n in &input.nodes {
        let _ = formatter.write_node(n);
    }

    if args.is_empty() {
        return quote! {
            ::vy::PreEscaped(#text)
        };
    }

    let vals = args.iter().map(|(_, v)| v);
    let pats = args
        .iter()
        .enumerate()
        .map(|(i, _)| format_ident!("__arg{i}"));
    let stmts = expand_statements(
        &text,
        args.iter().zip(pats.clone()).map(|((i, _), pat)| (*i, pat)),
    );
    let size_hint = input.size_hint;

    quote!(match (#(#vals),*) {
        (#(#pats),*) => {
            #[inline(always)]
            move |__buf: &mut String| {
                __buf.reserve(#size_hint);
                #(#stmts);*
            }
        }
    })
}

pub(crate) fn write(input: WriteInput) -> TokenStream {
    let mut text = String::new();
    let mut args = Vec::new();

    let mut formatter = Formatter::new(&mut text, &mut args);
    for n in &input.nodes {
        let _ = formatter.write_node(n);
    }

    let stmts = expand_statements(&text, args.into_iter());
    let buffer = input.buffer;
    let size_hint = input.size_hint;

    quote!({
        let __buf: &mut String = #buffer;
        __buf.reserve(#size_hint);
        #(#stmts);*
    })
}

fn expand_statements<T: ToTokens>(
    text: &str,
    args: impl Iterator<Item = (usize, T)>,
) -> Vec<TokenStream> {
    fn expand_str(s: &str) -> TokenStream {
        quote! {
            __buf.push_str(#s)
        }
    }

    fn expand_arg<T: ToTokens>(v: T) -> TokenStream {
        quote! {
            ::vy::Render::render_to(#v, __buf)
        }
    }

    let mut stmts = Vec::with_capacity(args.size_hint().0.saturating_mul(2));
    let mut cursor = 0usize;

    for (i, arg) in args {
        if i != cursor {
            stmts.push(expand_str(&text[cursor..i]));
            cursor = i;
        }
        stmts.push(expand_arg(arg));
    }

    if cursor < text.len() {
        stmts.push(expand_str(&text[cursor..]));
    }

    stmts
}
