#![no_std]

pub mod known;
pub mod prelude;

extern crate self as vy;
#[cfg(any(test, feature = "std"))]
extern crate std;

pub use vy_core::{escape, escape::PreEscaped, Attribute, Element, IntoHtml};
pub use vy_macros::{declare_elements, define_element, define_void_element};

#[cfg(test)]
mod tests {
    use vy::prelude::*;

    #[test]
    fn it_works() {
        let htmlz = meta!(http_equiv = "", svg::script!(href = ""));
        panic!("{}", htmlz.into_string());
    }
}
