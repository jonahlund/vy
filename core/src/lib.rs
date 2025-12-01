#![no_std]

mod escape;

pub use escape::PreEscaped;

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::string::String;

/// A type that can be represented as HTML.
pub trait IntoHtml: Sized {
    type Into: IntoHtml;

    fn into_html(self) -> Self::Into;

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        self.into_html().escape_and_write(buf);
    }

    #[inline]
    fn into_string(self) -> String {
        let inner = self.into_html();
        let size = inner.size_hint();
        let mut buf = String::with_capacity(size + (size / 10));
        inner.escape_and_write(&mut buf);
        buf
    }

    #[inline]
    fn size_hint(&self) -> usize {
        0
    }
}

impl IntoHtml for &str {
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        escape::escape_into(buf, self)
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl IntoHtml for bool {
    type Into = &'static str;

    #[inline]
    fn into_html(self) -> Self::Into {
        if self {
            "true"
        } else {
            "false"
        }
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.into_html().size_hint()
    }
}

impl IntoHtml for String {
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self::Into {
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

impl<T: IntoHtml> IntoHtml for Option<T> {
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self::Into {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        buf.reserve(self.size_hint());
        if let Some(x) = self {
            x.escape_and_write(buf)
        }
    }

    #[inline]
    fn size_hint(&self) -> usize {
        if let Some(x) = self {
            x.size_hint()
        } else {
            0
        }
    }
}

impl<B: IntoHtml, I: ExactSizeIterator, F> IntoHtml for core::iter::Map<I, F>
where
    F: FnMut(I::Item) -> B,
{
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self::Into {
        self
    }

    #[inline]
    fn escape_and_write(mut self, buf: &mut String) {
        let len = self.len();
        let x = self.next();
        buf.reserve(len * x.size_hint());
        for x in self {
            x.escape_and_write(buf);
        }
    }
}

impl<A: IntoHtml, B: IntoHtml> IntoHtml for (A, B) {
    type Into = (A::Into, B::Into);

    #[inline]
    fn into_html(self) -> Self::Into {
        (self.0.into_html(), self.1.into_html())
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        buf.reserve(self.size_hint());
        self.0.escape_and_write(buf);
        self.1.escape_and_write(buf);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.size_hint() + self.1.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use crate::IntoHtml;

    #[test]
    fn it_works() {
        assert_eq!(5, (false, true).size_hint());
    }
}
