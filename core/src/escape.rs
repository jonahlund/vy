use alloc::string::String;

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
    output.reserve(input.len());
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

/// A type that requires no further escaping.
pub struct PreEscaped<T>(pub T);

impl<T: AsRef<str>> IntoHtml for PreEscaped<T> {
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.as_ref().len()
    }
}
