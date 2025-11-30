#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::string::String;

/// A type that can be represented as HTML.
pub trait IntoHtml<T: IntoHtml = Self> {
    fn into_html(self) -> T;

    #[inline]
    fn peek(&self) -> Option<T> {
        None
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String)
    where
        Self: Sized,
    {
        self.into_html().escape_and_write(buf);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.peek().map(|inner| inner.size_hint()).unwrap_or(0)
    }
}

impl IntoHtml for &str {
    #[inline]
    fn into_html(self) -> Self {
        self
    }

    #[inline]
    fn peek(&self) -> Option<Self> {
        Some(self)
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        buf.push_str(self); // TODO: escape
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl IntoHtml<&'static str> for bool {
    #[inline]
    fn into_html(self) -> &'static str {
        if self {
            "true"
        } else {
            "false"
        }
    }

    #[inline]
    fn peek(&self) -> Option<&'static str> {
        Some(self.into_html())
    }
}

impl IntoHtml for String {
    #[inline]
    fn into_html(self) -> Self {
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

// impl<T: IntoHtml> IntoHtml<T> for core::iter::T {}

struct MyT {}

impl IntoHtml<impl IntoHtml> for MyT {
    fn into_html(self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::IntoHtml;

    #[test]
    fn it_works() {
        assert_eq!(5, false.size_hint());
    }
}
