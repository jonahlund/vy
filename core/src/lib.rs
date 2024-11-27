#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod escape;
mod helpers;

use alloc::string::String;

pub use escape::{escape_into, PreEscaped};
pub use helpers::{from_fn, FromFn};

/// A type that can be represented in HTML.
///
/// Some types implementing this trait (`&str`, `char`) are escaped by default.
/// To render types unescaped, use [`PreEscaped`].
///
/// [`PreEscaped`]: crate::PreEscaped
pub trait ToHtml {
    /// Writes the HTML into the given buffer.
    fn to_html(&self, buf: &mut String);

    /// Allocates a new `String` with the given HTML.
    #[inline]
    fn to_string(&self) -> String
    where
        Self: Sized,
    {
        let mut buf = String::new();
        self.to_html(&mut buf);
        buf
    }
}

impl<T: ToHtml + ?Sized> ToHtml for &T {
    #[inline]
    fn to_html(&self, buf: &mut String) {
        T::to_html(&**self, buf)
    }
}

impl<T: ToHtml + ?Sized> ToHtml for Box<T> {
    fn to_html(&self, buf: &mut String) {
        T::to_html(&**self, buf);
    }
}

macro_rules! via_itoap {
    ($($ty:ty)*) => {
        $(
            impl $crate::ToHtml for $ty {
                #[inline]
                fn to_html(&self, buf: &mut String) {
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
                fn to_html(&self, buf: &mut String) {
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
    fn to_html(&self, buf: &mut String) {
        escape_into(self, buf)
    }
}

impl ToHtml for String {
    #[inline]
    fn to_html(&self, buf: &mut String) {
        self.as_str().to_html(buf)
    }
}

impl ToHtml for char {
    #[inline]
    fn to_html(&self, buf: &mut String) {
        escape_into(self.encode_utf8(&mut [0; 4]), buf);
    }
}

impl ToHtml for bool {
    #[inline]
    fn to_html(&self, buf: &mut String) {
        buf.push_str(if *self { "true" } else { "false" })
    }
}

impl<T: ToHtml> ToHtml for Option<T> {
    #[inline]
    fn to_html(&self, buf: &mut String) {
        if let Some(x) = self {
            x.to_html(buf)
        }
    }
}

impl<T: ToHtml, const N: usize> ToHtml for [T; N] {
    #[inline]
    fn to_html(&self, buf: &mut String) {
        for x in self {
            x.to_html(buf)
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
			fn to_html(&self, buf: &mut String) {
				#[allow(non_snake_case)]
				let ($($i,)+) = self;
				$(
					$i.to_html(buf);
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
