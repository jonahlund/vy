#![doc = include_str!("../README.md")]

extern crate self as vy;

pub mod prelude {
    pub use vy_core::{either::*, escape::PreEscaped, *};
    pub use vy_macros::*;

    pub use crate::DOCTYPE;
}

pub use vy_core::{either::*, escape::PreEscaped, *};
pub use vy_macros::*;

pub const DOCTYPE: PreEscaped<&'static str> = PreEscaped("<!DOCTYPE html>");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_single_tags() {
        assert_eq!(a!().into_string(), "<a></a>");
        assert_eq!(header!().into_string(), "<header></header>");
        assert_eq!(fieldset!().into_string(), "<fieldset></fieldset>");
        assert_eq!(div!().into_string(), "<div></div>");
        assert_eq!(footer!().into_string(), "<footer></footer>");
        assert_eq!(h1!().into_string(), "<h1></h1>");
    }

    #[test]
    fn empty_multi_tags() {
        assert_eq!((a!(), header!()).into_string(), "<a></a><header></header>");
        assert_eq!(
            (div!(), span!(), span!(), span!(), div!()).into_string(),
            "<div></div><span></span><span></span><span></span><div></div>"
        );
    }

    #[test]
    fn nested_single_tags() {
        assert_eq!(div!(span!()).into_string(), "<div><span></span></div>");
        assert_eq!(span!(h1!()).into_string(), "<span><h1></h1></span>");
        assert_eq!(
            html!(body!(div!())).into_string(),
            "<html><body><div></div></body></html>"
        );
        assert_eq!(
            div!(div!(div!(div!(span!(div!()))))).into_string(),
            "<div><div><div><div><span><div></div></span></div></div></div></\
             div>"
        );
    }

    #[test]
    fn nested_multi_tags() {
        assert_eq!(
            html!(head!(title!()), body!(div!(div!(div!()), div!(span!()))))
                .into_string(),
            "<html><head><title></title></head><body><div><div><div></div></\
             div><div><span></span></div></div></body></html>"
        );
    }

    #[test]
    fn void_tags() {
        assert_eq!(area!().into_string(), "<area>");
        assert_eq!(base!().into_string(), "<base>");
        assert_eq!(br!().into_string(), "<br>");
        assert_eq!(col!().into_string(), "<col>");
        assert_eq!(embed!().into_string(), "<embed>");
        assert_eq!(hr!().into_string(), "<hr>");
        assert_eq!(img!().into_string(), "<img>");
        assert_eq!(input!().into_string(), "<input>");
        assert_eq!(link!().into_string(), "<link>");
        assert_eq!(meta!().into_string(), "<meta>");
        assert_eq!(source!().into_string(), "<source>");
        assert_eq!(track!().into_string(), "<track>");
        assert_eq!(wbr!().into_string(), "<wbr>");
    }

    #[test]
    fn attributes() {
        assert_eq!(
            div!(class = "foo bar", id = "baz").into_string(),
            "<div class=\"foo bar\" id=\"baz\"></div>"
        );
    }
}
