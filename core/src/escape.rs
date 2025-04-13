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

pub struct PreEscaped<T>(pub T);

impl IntoHtml for PreEscaped<&str> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        buf.push_str(self.0);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.size_hint()
    }
}

impl IntoHtml for PreEscaped<String> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        buf.push_str(&self.0);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.size_hint()
    }
}

impl IntoHtml for PreEscaped<char> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        buf.push(self.0);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.len_utf8()
    }
}
