//! Fast and minimal HTML macros in Rust.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate self as vy;

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
/// fn button(label: impl vy::Render) -> impl vy::Render {
///     vy::lazy! {
///         <button>{label}</button>
///     }
/// }
/// ```
#[doc(inline)]
pub use vy_macros::lazy;
/// Writes HTML to a [`String`].
///
/// This macro eagerly writes HTML to a [`String`].
#[doc(inline)]
pub use vy_macros::write;

/// Render HTML to a [`String`].
///
/// This is a convenience macro over `vy::lazy!().render()`.
#[macro_export]
macro_rules! render {
    ($($arg:tt)*) => {
        vy::Render::render(vy::lazy!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    #[test]
    fn simple_tags() {
        assert_eq!(vy::render!(<foo></foo>), "<foo></foo>");
        assert_eq!(
            vy::render!(<foo></foo><bar></bar>),
            "<foo></foo><bar></bar>"
        );
    }

    #[test]
    fn simple_tags_with_attributes() {
        assert_eq!(
            vy::render!(<foo bar="baz"></foo>),
            "<foo bar=\"baz\"></foo>"
        );
        assert_eq!(
            vy::render!(<foo bar="baz" qux={false}></foo>),
            "<foo bar=\"baz\" qux=\"false\"></foo>"
        );
    }

    #[test]
    fn nested_tags() {
        assert_eq!(
            vy::render!(<foo><bar></bar></foo>),
            "<foo><bar></bar></foo>"
        );
        assert_eq!(
            vy::render!(<foo><bar><baz></baz></bar></foo><qux></qux>),
            "<foo><bar><baz></baz></bar></foo><qux></qux>"
        );
    }

    #[test]
    fn self_closing_tags() {
        assert_eq!(vy::render!(<foo />), "<foo>");
        assert_eq!(vy::render!(<foo /><bar />), "<foo><bar>");
    }
}