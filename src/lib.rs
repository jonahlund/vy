//! Fast and minimal HTML templating macros.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate self as vy;

pub use vy_core::{from_fn, FromFn, PreEscaped, ToHtml};
/// Creates a renderable type.
///
/// This avoids allocating until [`to_string`] is called, which makes it
/// suitable to use whenever you are simply passing along the result for
/// further rendering.
///
/// [`to_string`]: crate::ToHtml::to_string
///
/// # Example
///
/// ```
/// fn button(label: impl vy::ToHtml) -> impl vy::ToHtml {
///     vy::lazy! {
///         <button>{label}</button>
///     }
/// }
/// ```
#[doc(inline)]
pub use vy_macros::lazy;
/// Creates an HTML string literal.
///
/// Passing any non literal values will cause a compilation error.
#[doc(inline)]
pub use vy_macros::literal;
/// Writes HTML to a [`String`].
#[doc(inline)]
pub use vy_macros::write;

/// Creates a [`String`] with the HTML content.
///
/// This is a convenience macro over `vy::lazy!(..).to_string()`.
#[macro_export]
macro_rules! owned {
    ($($arg:tt)*) => {
        $crate::ToHtml::to_string(&$crate::lazy!($($arg)*))
    };
}

/// Inline script tag
///
/// # Usage
/// ```
/// let page = vy::owned! {
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
        $crate::PreEscaped(concat!("<script>", stringify!($($t)*), "</script>"))
    };
}

/// Inline style tag
///
/// # Usage
///
/// ```
/// let page = vy::owned! {
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
        $crate::PreEscaped(concat!("<style>", stringify!($($t)*), "</style>"))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_tags() {
        assert_eq!(vy::owned!(<foo></foo>), "<foo></foo>");
        assert_eq!(
            vy::owned!(<foo></foo><bar></bar>),
            "<foo></foo><bar></bar>"
        );
    }

    #[test]
    fn simple_tags_with_attributes() {
        assert_eq!(
            vy::owned!(<foo bar="baz"></foo>),
            "<foo bar=\"baz\"></foo>"
        );
        assert_eq!(
            vy::owned!(<foo bar="baz" qux={false}></foo>),
            "<foo bar=\"baz\" qux=\"false\"></foo>"
        );
    }

    #[test]
    fn nested_tags() {
        assert_eq!(
            vy::owned!(<foo><bar></bar></foo>),
            "<foo><bar></bar></foo>"
        );
        assert_eq!(
            vy::owned!(<foo><bar><baz></baz></bar></foo><qux></qux>),
            "<foo><bar><baz></baz></bar></foo><qux></qux>"
        );
    }

    #[test]
    fn self_closing_tags() {
        assert_eq!(vy::owned!(<foo />), "<foo>");
        assert_eq!(vy::owned!(<foo /><bar />), "<foo><bar>");
    }
}
