use alloc::string::String;
use core::fmt::Write as _;

use crate::Render;

/// Escapes all special HTML characters in `input` and returns the result.
///
/// The following characters are escaped:
///
///   '&' -> `&amp;`
///   '<' -> `&lt;`
///   '>' -> `&gt;`
///   '"' -> `&quot;`
///
/// All other characters remain unchanged.
#[inline]
pub fn escape(input: &str) -> String {
    let mut buf = String::with_capacity(input.len());
    escape_into(input, &mut buf);
    buf
}

/// Escapes all special HTML characters in `input` and writes the result into
/// `buf`.
#[inline]
pub fn escape_into(input: &str, buf: &mut String) {
    for c in input.chars() {
        match c {
            '&' => buf.push_str("&amp;"),
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            '"' => buf.push_str("&quot;"),
            _ => buf.push(c),
        };
    }
}

/// A type that is assumed to be pre-escaped and shouldn't require further
/// escaping.
pub struct PreEscaped<T: ?Sized>(pub T);

impl Render for PreEscaped<&str> {
    #[inline]
    fn render_to(self, buf: &mut String) {
        buf.push_str(self.0);
    }
}

impl Render for PreEscaped<String> {
    #[inline]
    fn render_to(self, buf: &mut String) {
        buf.push_str(&self.0);
    }
}

impl Render for PreEscaped<core::fmt::Arguments<'_>> {
    fn render_to(self, buf: &mut String) {
        let _ = buf.write_fmt(self.0);
    }
}
