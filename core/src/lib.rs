#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod helpers;
mod escape;

use alloc::string::String;

pub use escape::*;

/// A type that can be rendered to a [`String`].
///
/// Some types implementing this trait (`&str`, `char`) are escaped by default.
/// To render types unescaped, use [`PreEscaped`].
///
/// [`PreEscaped`]: crate::PreEscaped
pub trait Render {
    fn render_to(self, buf: &mut String);

    #[inline]
    fn render(self) -> String
    where
        Self: Sized,
    {
        let mut buf = String::new();
        self.render_to(&mut buf);
        buf
    }
}

via_itoap! {
    isize i8 i16 i32 i64 i128
    usize u8 u16 u32 u64 u128
}

via_ryu! { f32 f64 }

impl Render for &str {
    #[inline]
    fn render_to(self, buf: &mut String) {
        escape_into(self, buf)
    }
}

impl Render for String {
    #[inline]
    fn render_to(self, buf: &mut String) {
        self.as_str().render_to(buf)
    }
}

impl Render for &String {
    #[inline]
    fn render_to(self, buf: &mut String) {
        self.as_str().render_to(buf)
    }
}

#[cfg(feature = "std")]
impl Render for Box<str> {
    #[inline]
    fn render_to(self, buf: &mut String) {
        self.as_ref().render_to(buf);
    }
}

impl Render for char {
    #[inline]
    fn render_to(self, buf: &mut String) {
        escape_into(self.encode_utf8(&mut [0; 4]), buf);
    }
}

impl Render for bool {
    #[inline]
    fn render_to(self, buf: &mut String) {
        buf.push_str(if self { "true" } else { "false" })
    }
}

impl<F: FnOnce(&mut String)> Render for F {
    #[inline]
    fn render_to(self, buf: &mut String) {
        (self)(buf)
    }
}

impl<T: Render> Render for Option<T> {
    #[inline]
    fn render_to(self, buf: &mut String) {
        if let Some(x) = self {
            x.render_to(buf)
        }
    }
}

impl<T: Render, const N: usize> Render for [T; N] {
    #[inline]
    fn render_to(self, buf: &mut String) {
        for x in self {
            x.render_to(buf)
        }
    }
}

impl<T, I: Iterator, F> Render for core::iter::Map<I, F>
where
    T: Render,
    F: FnMut(I::Item) -> T,
{
    #[inline]
    fn render_to(self, buf: &mut String) {
        for x in self {
            x.render_to(buf)
        }
    }
}

impl<T: Render> Render for alloc::vec::IntoIter<T> {
    #[inline]
    fn render_to(self, buf: &mut String) {
        for x in self {
            x.render_to(buf)
        }
    }
}
