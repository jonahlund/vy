//! Fast and minimal HTML macros in Rust.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate self as vyu;

pub mod runtime;
pub use runtime::{PreEscaped, Render};
/// Lazily write HTML.
///
/// This avoids allocating until [`render`] is called, which makes it
/// suitable to use whenever you are simply passing along the result for
/// further formatting.
///
/// [`render`]: crate::Render
///
/// # Example
///
/// ```
/// fn button(label: impl vyu::Render) -> impl vyu::Render {
///     vyu::lazy! {
///         <button>{label}</button>
///     }
/// }
/// ```
#[doc(inline)]
pub use vyu_macros::lazy;
/// Writes HTML to a [`String`].
///
/// This macro eagerly writes HTML to a [`String`].
#[doc(inline)]
pub use vyu_macros::write;

/// Render HTML to a [`String`].
///
/// This is a convenience macro over `vyu::lazy!().render()`.
#[macro_export]
macro_rules! render {
    ($($arg:tt)*) => {
        vyu::Render::render(vyu::lazy!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    #[test]
    fn simple_tags() {
        assert_eq!(vyu::render!(<foo></foo>), "<foo></foo>");
        assert_eq!(
            vyu::render!(<foo></foo><bar></bar>),
            "<foo></foo><bar></bar>"
        );
    }

    #[test]
    fn simple_tags_with_attributes() {
        assert_eq!(
            vyu::render!(<foo bar="baz"></foo>),
            "<foo bar=\"baz\"></foo>"
        );
        assert_eq!(
            vyu::render!(<foo bar="baz" qux={false}></foo>),
            "<foo bar=\"baz\" qux=\"false\"></foo>"
        );
    }

    #[test]
    fn nested_tags() {
        assert_eq!(
            vyu::render!(<foo><bar></bar></foo>),
            "<foo><bar></bar></foo>"
        );
        assert_eq!(
            vyu::render!(<foo><bar><baz></baz></bar></foo><qux></qux>),
            "<foo><bar><baz></baz></bar></foo><qux></qux>"
        );
    }

    #[test]
    fn self_closing_tags() {
        assert_eq!(vyu::render!(<foo />), "<foo>");
        assert_eq!(vyu::render!(<foo /><bar />), "<foo><bar>");
    }
}
