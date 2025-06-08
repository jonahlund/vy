use alloc::string::String;

use crate::IntoHtml;

/// Returns the escaped encoding of `ch`, if any.
#[inline]
pub const fn escape_char(ch: char) -> Option<&'static str> {
    match ch {
        '&' => Some("&amp;"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&#39;"),
        _ => None,
    }
}

/// Escapes special HTML characters in `text` and writes the result into `buf`.
#[inline]
pub fn escape_into(buf: &mut String, text: &str) {
    buf.reserve(text.len());
    for ch in text.chars() {
        match escape_char(ch) {
            Some(escaped) => {
                buf.push_str(escaped);
            }
            None => {
                buf.push(ch);
            }
        }
    }
}

/// Escapes special HTML characters in `text`.
#[inline]
pub fn escape(text: &str) -> String {
    let mut res = String::new();
    escape_into(&mut res, text);
    res
}

/// A type assumed to be escaped.
///
/// The [`IntoHtml`] implementation for this type will not perform additional
/// escaping.
pub struct PreEscaped<T>(pub T);

impl<T: AsRef<str>> IntoHtml for PreEscaped<T> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
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
