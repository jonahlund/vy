extern crate self as vy;

pub mod builtin;
#[macro_use]
pub mod well_known;

pub use builtin::{
    Attribute, Either, Either3, Either4, Either5, Either6, Either7, Either8,
    Either9, OptionalAttribute, SpreadAttributes,
};
pub use vy_core::{IntoHtml, PreEscaped};
pub use vy_macros::_compile;

#[macro_export]
macro_rules! _tag {
    (@diagnostic $($tt:tt)*) => {};
    ($($tt:tt)*) => {
        $crate::_compile!(_tag!($($tt)*))
    };
}

#[macro_export]
macro_rules! _void_tag {
    (@diagnostic $($tt:tt)*) => {};
    ($($tt:tt)*) => {
        $crate::_compile!(_void_tag!($($tt)*))
    };
}

#[macro_export]
macro_rules! _frag {
    (@diagnostic $($tt:tt)*) => {};
    ($($tt:tt)*) => {
        $crate::_compile!($($tt)*)
    };
}

#[macro_export]
macro_rules! _if {
    (@diagnostic $($tt:tt)*) => {};
    ($($tt:tt)*) => {
        $crate::_compile!(if $($tt)*)
    };
}

#[macro_export]
macro_rules! _match {
    (@diagnostic $($tt:tt)*) => {};
    ($($tt:tt)*) => {
        $crate::_compile!(match $($tt)*)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
