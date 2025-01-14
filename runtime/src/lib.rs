#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod escape;
mod helpers;

use alloc::{boxed::Box, string::String};

pub use crate::{
    escape::{Escape, PreEscaped, escape, escape_char, escape_into},
    helpers::{FromFn, from_fn},
};

/// A type that can be represented in HTML.
///
/// Some types implementing this trait (`&str`, `char`) are escaped by default.
/// To render types unescaped, use [`PreEscaped`].
///
/// [`PreEscaped`]: crate::PreEscaped
pub trait ToHtml {
    /// Writes the HTML into the given buffer.
    fn write_escaped(&self, buf: &mut String);

    /// Allocates a new `String` with the given HTML.
    #[inline]
    fn to_string(&self) -> String
    where
        Self: Sized,
    {
        let mut buf = String::new();
        self.write_escaped(&mut buf);
        buf
    }
}

impl<T: ToHtml + ?Sized> ToHtml for &T {
    #[inline]
    fn write_escaped(&self, buf: &mut String) {
        T::write_escaped(&**self, buf)
    }
}

impl<T: ToHtml + ?Sized> ToHtml for Box<T> {
    fn write_escaped(&self, buf: &mut String) {
        T::write_escaped(&**self, buf);
    }
}

macro_rules! via_itoap {
    ($($ty:ty)*) => {
        $(
            impl $crate::ToHtml for $ty {
                #[inline]
                fn write_escaped(&self, buf: &mut String) {
                    itoap::write_to_string(buf, *self)
                }
            }
        )*
    };
}

macro_rules! via_ryu {
    ($($ty:ty)*) => {
        $(
            impl $crate::ToHtml for $ty {
                #[inline]
                fn write_escaped(&self, buf: &mut String) {
                    buf.push_str(ryu::Buffer::new().format(*self));
                }
            }
        )*
    };
}

via_itoap! {
    isize i8 i16 i32 i64 i128
    usize u8 u16 u32 u64 u128
}

via_ryu! { f32 f64 }

impl ToHtml for str {
    #[inline]
    fn write_escaped(&self, buf: &mut String) {
        escape_into(buf, self)
    }
}

impl ToHtml for String {
    #[inline]
    fn write_escaped(&self, buf: &mut String) {
        self.as_str().write_escaped(buf)
    }
}

impl ToHtml for char {
    #[inline]
    fn write_escaped(&self, buf: &mut String) {
        escape_into(buf, self.encode_utf8(&mut [0; 4]));
    }
}

impl ToHtml for bool {
    #[inline]
    fn write_escaped(&self, buf: &mut String) {
        buf.push_str(if *self { "true" } else { "false" })
    }
}

impl<T: ToHtml> ToHtml for Option<T> {
    #[inline]
    fn write_escaped(&self, buf: &mut String) {
        if let Some(x) = self {
            x.write_escaped(buf)
        }
    }
}

impl<T: ToHtml, const N: usize> ToHtml for [T; N] {
    #[inline]
    fn write_escaped(&self, buf: &mut String) {
        for x in self {
            x.write_escaped(buf)
        }
    }
}

macro_rules! impl_tuple {
	((
		$($i:ident,)+
	)) => {
		impl<$($i,)+> ToHtml for ($($i,)+)
		where
			$($i: ToHtml,)+
		{
			fn write_escaped(&self, buf: &mut String) {
				#[allow(non_snake_case)]
				let ($($i,)+) = self;
				$(
					$i.write_escaped(buf);
				)+
			}
		}
	};

	($f:ident) => {
		impl_tuple!(($f,));
	};

	($f:ident $($i:ident)+) => {
		impl_tuple!(($f, $($i,)+));
		impl_tuple!($($i)+);
	};
}

impl_tuple!(A B C D E F G H I J K);
