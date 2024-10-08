#[macro_use]
mod helpers;
mod escape;
mod size_hint;

use alloc::string::String;

pub use escape::*;
pub use size_hint::*;

/// A type that can be rendered to a [`String`].
///
/// Some types implementing this trait (`&str`, `char`) are escaped by default.
/// To render types unescaped, use [`PreEscaped`].
///
/// [`PreEscaped`]: crate::PreEscaped
///
/// # Example
///
/// ```
/// struct AppleCount(i32);
///
/// impl vy::Render for AppleCount {
///     fn render_to(self, buf: &mut String) {
///         let suffix = if self.0 != 1 { "s" } else { "" };
///         
///         vy::write!(buf,
///             <p>"You have "{self.0}" apple"{suffix}"!"<p>
///         )
///     }
/// }
/// ```
pub trait Render {
    fn render_to(self, buf: &mut String);

    fn render(self) -> String
    where
        Self: Sized,
    {
        static SIZE_HINT: SizeHint = SizeHint::new(0);
        let mut buf = String::with_capacity(SIZE_HINT.get());
        self.render_to(&mut buf);
        SIZE_HINT.update(buf.len());
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
    fn render_to(self, buf: &mut String) {
        self.as_str().render_to(buf)
    }
}

impl Render for bool {
    fn render_to(self, buf: &mut String) {
        buf.push_str(if self { "true" } else { "false" })
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

impl<T, I: Iterator, F> Render for core::iter::Map<I, F>
where
    T: Render,
    F: FnMut(I::Item) -> T,
{
    fn render_to(self, buf: &mut String) {
        for x in self {
            x.render_to(buf)
        }
    }
}

impl<T: Render> Render for alloc::vec::IntoIter<T> {
    fn render_to(self, buf: &mut String) {
        for x in self {
            x.render_to(buf)
        }
    }
}
