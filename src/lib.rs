#![allow(clippy::len_without_is_empty)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub mod concat;
mod expand;

pub use vy_macros::*;
pub use vy_runtime::*;

pub const DOCTYPE: PreEscaped<&str> = PreEscaped("<!DOCTYPE html>");

#[macro_export]
macro_rules! lit {
    ($($x:tt)*) => {
        $crate::into!($crate::x_literal, $($x)*)
    };
}

#[macro_export]
macro_rules! owned {
    ($($x:tt)*) => {
        $crate::into!($crate::x_string, $($x)*)
    };
}

#[macro_export]
macro_rules! lazy {
    ($($x:tt)*) => {
        $crate::into!($crate::x_closure, $($x)*)
    };
}
