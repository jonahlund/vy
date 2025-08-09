#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod buffer;
pub mod either;
pub mod escape;
mod helpers;

use alloc::string::String;

pub use self::buffer::Buffer;
use self::escape::escape_into;

/// A type that can be represented as HTML.
pub trait IntoHtml {
    /// Converts this value into HTML by producing a type that implements
    /// [`IntoHtml`].
    ///
    /// This method enables composition of HTML structures by delegating
    /// rendering to the returned value. Use it to build nested HTML
    /// elements, combine components, or leverage existing [`IntoHtml`]
    /// implementations.
    ///
    /// # Examples
    ///
    /// Compose nested HTML elements using macros:
    ///
    /// ```
    /// # use vy::*;
    /// struct Article {
    ///     title: String,
    ///     content: String,
    ///     author: String,
    /// }
    ///
    /// impl IntoHtml for Article {
    ///     fn into_html(self) -> impl IntoHtml {
    ///         article!(
    ///             h1!(self.title),
    ///             p!(class = "content", self.content),
    ///             footer!("Written by ", self.author)
    ///         )
    ///     }
    /// }
    /// ```
    ///
    /// Chain multiple implementations through delegation:
    ///
    /// ```
    /// # use vy::*;
    /// # struct Article;
    /// # impl IntoHtml for Article {
    /// #     fn into_html(self) -> impl IntoHtml {}
    /// # }
    /// struct ArticlePage {
    ///     title: String,
    ///     articles: Vec<Article>,
    /// }
    ///
    /// impl IntoHtml for ArticlePage {
    ///     fn into_html(self) -> impl IntoHtml {
    ///         html!(head!(title!(self.title)), body!(self.articles))
    ///     }
    /// }
    /// ```
    ///
    /// For "leaf" types (elements that render directly without children, like
    /// primitive values), **always return `self`** to avoid infinite recursion:
    ///
    /// ```
    /// # use vy::{prelude::*, escape::escape_into};
    /// struct TextNode(String);
    ///
    /// impl IntoHtml for TextNode {
    ///     fn into_html(self) -> impl IntoHtml {
    ///         // Leaf type returns itself to terminate the rendering chain
    ///         self
    ///     }
    ///
    ///     fn escape_and_write(self, buf: &mut Buffer) {
    ///         escape_into(buf, &self.0);
    ///     }
    ///
    ///     fn size_hint(&self) -> usize {
    ///         self.0.len()
    ///     }
    /// }
    /// ```
    fn into_html(self) -> impl IntoHtml;

    /// Writes the HTML into the provided [`Buffer`].
    #[inline]
    fn escape_and_write(self, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.into_html().escape_and_write(buf);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        0
    }

    /// Allocates a new [`String`] containing the HTML.
    fn into_string(self) -> String
    where
        Self: Sized,
    {
        let html = self.into_html();
        let size = html.size_hint();
        let mut buf = Buffer::with_capacity(size + (size / 10));
        html.escape_and_write(&mut buf);
        buf.into_string()
    }
}

impl IntoHtml for &str {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut Buffer) {
        escape_into(buf, self)
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
    fn escape_and_write(self, buf: &mut Buffer) {
        escape_into(buf, self.encode_utf8(&mut [0; 4]));
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len_utf8()
    }
}

impl IntoHtml for String {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut Buffer) {
        escape_into(buf, &self)
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl IntoHtml for &String {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self.as_str()
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
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
        5
    }
}

impl<T: IntoHtml> IntoHtml for Option<T> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut Buffer) {
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

impl IntoHtml for () {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, _: &mut Buffer) {}
}

impl<F: FnOnce(&mut Buffer)> IntoHtml for F {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut Buffer) {
        (self)(buf)
    }
}

impl<B: IntoHtml, I: ExactSizeIterator, F> IntoHtml for core::iter::Map<I, F>
where
    F: FnMut(I::Item) -> B,
{
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut Buffer) {
        let len = self.len();
        for (i, x) in self.enumerate() {
            if i == 0 {
                buf.reserve(len * x.size_hint());
            }
            x.escape_and_write(buf);
        }
    }
}

impl<T: IntoHtml> IntoHtml for alloc::vec::Vec<T> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut Buffer) {
        for x in self {
            x.escape_and_write(buf);
        }
    }

    #[inline]
    fn size_hint(&self) -> usize {
        let mut n = 0;
        for x in self {
            n += x.size_hint();
        }
        n
    }
}

impl<T: IntoHtml, const N: usize> IntoHtml for [T; N] {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut Buffer) {
        for x in self {
            x.escape_and_write(buf);
        }
    }

    #[inline]
    fn size_hint(&self) -> usize {
        let mut n = 0;
        for x in self {
            n += x.size_hint();
        }
        n
    }
}

impl<'a> IntoHtml for alloc::borrow::Cow<'a, str> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut Buffer) {
        escape_into(buf, self.as_ref())
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.as_ref().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_cow_str() {
        use alloc::borrow::Cow;

        assert_eq!(
            <Cow<'static, str> as IntoHtml>::into_string(Cow::Borrowed(
                "borrowed"
            )),
            "borrowed"
        );
        assert_eq!(
            <Cow<'static, str> as IntoHtml>::into_string(Cow::Owned(
                String::from("owned")
            )),
            "owned"
        );
    }
}
