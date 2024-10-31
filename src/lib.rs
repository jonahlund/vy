//! Fast and minimal HTML templating macros.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate self as vy;

pub use vy_core::{PreEscaped, Render};
/// Creates a renderable type.
///
/// This avoids allocating until [`render`] is called, which makes it
/// suitable to use whenever you are simply passing along the result for
/// further rendering.
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

/// Creates a [`String`] with the HTML content.
///
/// This is a convenience macro over `vy::lazy!(..).render()`.
#[macro_export]
macro_rules! render {
    ($($arg:tt)*) => {
        vy::Render::render(vy::lazy!($($arg)*))
    };
}

/// Inline script tag
///
/// # Usage
/// ```
/// let page = vy::render! {
///    {vy::script!(
///        console.log("Hello,");
///        console.log("world!");
///    )}
/// };
/// assert_eq!(
///     page,
///     r#"<script>console.log("Hello,"); console.log("world!");</script>"#
/// );
/// ```
#[macro_export]
macro_rules! script {
    ($($t:tt)*) => {
        vy::PreEscaped(concat!("<script>", stringify!($($t)*), "</script>"))
    };
}

/// Inline style tag
///
/// # Usage
///
/// ```
/// let page = vy::render! {
///    {vy::style!(
///        .red { color: red }
///        .green { color: green }
///    )}
/// };
/// assert_eq!(
///     page,
///     "<style>.red { color: red }.green { color: green }</style>"
/// );
/// ```
#[macro_export]
macro_rules! style {
    ($($t:tt)*) => {
        vy::PreEscaped(concat!("<style>", stringify!($($t)*), "</style>"))
    };
}

#[cfg(test)]
mod tests {
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
