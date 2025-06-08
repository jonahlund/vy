#![no_std]

pub mod known;
pub mod prelude;

extern crate self as vy;
#[cfg(any(test, feature = "std"))]
extern crate std;

pub use vy_core::{escape, escape::PreEscaped, IntoHtml};
pub use vy_macros::*;

fn t() {
    const X: PreEscaped<&str> =
        div!(class = "", id = 123, aria_label = "", div!(id = ""));
}
