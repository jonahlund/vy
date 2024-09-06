//! A fast and minimal HTML templating library in Rust.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate self as vy;

mod escape;
#[macro_use]
mod helpers;

pub use vy_macros::*;

use self::escape::escape_into;
pub use self::escape::{escape, PreEscaped};

pub trait Render {
    fn render_to(self, buf: &mut String);

    fn render(self) -> String
    where
        Self: Sized,
    {
        let mut buf = String::new();
        self.render_to(&mut buf);
        buf
    }
}

via_itoa! {
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
    fn render_to(self, buf: &mut String) {
        self.as_str().render_to(buf)
    }
}

impl<F: FnOnce(&mut String)> Render for F {
    fn render_to(self, buf: &mut String) {
        (self)(buf)
    }
}

impl<T: Render> Render for Option<T> {
    fn render_to(self, buf: &mut String) {
        if let Some(x) = self {
            x.render_to(buf)
        }
    }
}

impl<T: Render, const N: usize> Render for [T; N] {
    fn render_to(self, buf: &mut String) {
        for x in self {
            x.render_to(buf)
        }
    }
}

impl<T: Render> Render for std::vec::IntoIter<T> {
    fn render_to(self, buf: &mut String) {
        for x in self {
            x.render_to(buf)
        }
    }
}
