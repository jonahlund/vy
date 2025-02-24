use alloc::string::String;
use core::fmt::Write as _;

use crate::IntoHtml;

#[inline]
pub const fn escape_char(ch: char) -> Option<&'static str> {
    match ch {
        '&' => Some("&amp;"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        _ => None,
    }
}

/// Escapes all special HTML characters in `input` and writes the result into
/// `buf`.
#[inline]
pub fn escape_into(output: &mut String, input: &str) {
    for ch in input.chars() {
        match escape_char(ch) {
            Some(esc) => output.push_str(esc),
            _ => output.push(ch),
        };
    }
}

/// Escapes all special HTML characters in `input`.
#[inline]
pub fn escape(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    escape_into(&mut output, input);
    output
}

pub struct Escape<T>(pub T);

pub struct PreEscaped<T>(pub T);

impl IntoHtml for PreEscaped<&str> {
    #[inline]
    fn write_escaped(self, buf: &mut String) {
        buf.push_str(self.0);
    }
}

impl IntoHtml for PreEscaped<String> {
    #[inline]
    fn write_escaped(self, buf: &mut String) {
        buf.push_str(&self.0);
    }
}

impl IntoHtml for PreEscaped<core::fmt::Arguments<'_>> {
    #[inline]
    fn write_escaped(self, buf: &mut String) {
        let _ = buf.write_fmt(self.0);
    }
}
