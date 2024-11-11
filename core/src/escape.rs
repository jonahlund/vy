use alloc::string::String;
use core::fmt::Write as _;

use crate::ToHtml;

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

impl ToHtml for PreEscaped<&str> {
    #[inline]
    fn to_html(&self, buf: &mut String) {
        buf.push_str(self.0);
    }
}

impl ToHtml for PreEscaped<String> {
    #[inline]
    fn to_html(&self, buf: &mut String) {
        buf.push_str(&self.0);
    }
}

impl ToHtml for PreEscaped<core::fmt::Arguments<'_>> {
    #[inline]
    fn to_html(&self, buf: &mut String) {
        let _ = buf.write_fmt(self.0);
    }
}
