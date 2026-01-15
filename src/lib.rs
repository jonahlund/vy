extern crate self as vy;

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

async fn get_user() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
