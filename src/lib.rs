extern crate self as vy;

pub use vy_core::{PreEscaped, ToHtml};
pub use vy_macros::vy;

#[doc(hidden)]
#[allow(non_camel_case_types)]
pub mod __private {
    pub struct div;
    pub struct input;

    #[macro_export]
    macro_rules! div {
        (@__vy_macro) => {};
        ($($tt:tt)*) => {
            $crate::vy!(div!($($tt)*))
        }
    }

    #[macro_export]
    macro_rules! input {
        (@__vy_macro) => {};
        ($($tt:tt)*) => {
            $crate::vy!(input!($($tt)*))
        }
    }
}
