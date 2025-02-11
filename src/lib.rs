#![allow(clippy::len_without_is_empty)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![no_std]

extern crate alloc;
extern crate self as vy;
#[cfg(feature = "std")]
extern crate std;

pub mod concat;

pub use vy_core::{from_fn, Escape, FromFn, PreEscaped, ToHtml};
#[doc(hidden)]
pub use vy_macros::closure;
pub use vy_macros::forward;

#[macro_export]
macro_rules! lit {
    ($($tt:tt)*) => {
        $crate::forward!($crate::str, $($tt)*)
    };
}

#[macro_export]
macro_rules! lazy {
    ($($tt:tt)*) => {
        $crate::forward!($crate::closure, $($tt)*)
    };
}

#[macro_export]
macro_rules! owned {
    ($($tt:tt)*) => {
        $crate::forward!($crate::string, $($tt)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! str {
    ($($arg:expr),*) => {
        $crate::PreEscaped($crate::concat!($(
            $crate::str!(@stmt $arg)
        ),*))
    };
    (@stmt $arg:literal) => {
        $arg
    };
    (@stmt $arg:expr) => {
        $crate::Escape($arg)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! string {
    ($($arg:expr),*) => {
        $crate::ToHtml::to_string(&$crate::closure($($arg),*))
    };
}
