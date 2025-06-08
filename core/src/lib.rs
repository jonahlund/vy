#![no_std]

#[cfg(any(test, feature = "std"))]
extern crate std;

extern crate alloc;

pub mod escape;

use alloc::string::String;

use crate::escape::escape_into;

/// A type that can be represented as HTML.
pub trait IntoHtml {
    fn into_html(self) -> impl IntoHtml;

    /// Escapes this type and writes it into `buf`.
    #[inline]
    fn escape_and_write(self, buf: &mut String)
    where
        Self: Sized,
    {
        let html = self.into_html();
        buf.reserve(html.size_hint());
        html.escape_and_write(buf);
    }

    /// Returns an estimated size of the HTML representation for this type.
    #[inline]
    fn size_hint(&self) -> usize {
        0
    }

    #[inline]
    fn into_string(self) -> String
    where
        Self: Sized,
    {
        let html = self.into_html();
        let mut buf = String::with_capacity(html.size_hint());
        html.escape_and_write(&mut buf);
        buf
    }
}

impl IntoHtml for &str {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        escape::escape_into(buf, self);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl IntoHtml for String {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        self.as_str().escape_and_write(buf);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl IntoHtml for &String {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        self.as_str().escape_and_write(buf);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl IntoHtml for char {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        escape_into(buf, self.encode_utf8(&mut [0; 4]));
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len_utf8()
    }
}

impl IntoHtml for bool {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        if self {
            "true"
        } else {
            "false"
        }
    }

    #[inline]
    fn size_hint(&self) -> usize {
        if *self {
            "true".len()
        } else {
            "false".len()
        }
    }
}
