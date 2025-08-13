use alloc::{borrow::Cow, string::String};

use crate::{buffer::Buffer, IntoHtml};

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
pub fn escape_into(output: &mut Buffer, input: &str) {
    for ch in input.chars() {
        match escape_char(ch) {
            Some(esc) => output.push_str(esc),
            _ => output.push(ch),
        };
    }
}

/// Escapes all special HTML characters in `input`.
#[inline]
pub fn escape(input: &str) -> Buffer {
    let mut output = Buffer::with_capacity(input.len());
    escape_into(&mut output, input);
    output
}

/// A type that requires no further escaping.
pub struct PreEscaped<T>(pub T);

impl IntoHtml for PreEscaped<&str> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut Buffer) {
        buf.push_str(self.0);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.len()
    }
}

impl IntoHtml for PreEscaped<String> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut Buffer) {
        buf.push_str(&self.0);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.len()
    }
}

impl IntoHtml for PreEscaped<char> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut Buffer) {
        buf.push(self.0);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.len_utf8()
    }
}

impl IntoHtml for PreEscaped<Cow<'static, str>> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut Buffer) {
        buf.push_str(&self.0);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preescaped_cow_str() {
        assert_eq!(
            PreEscaped(Cow::Borrowed("<b>borrowed</b>")).into_string(),
            "<b>borrowed</b>"
        );
        assert_eq!(
            PreEscaped(Cow::Owned(String::from("<b>owned</b>"))).into_string(),
            "<b>owned</b>"
        );
    }
}
