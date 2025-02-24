#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod escape;
mod helpers;

use alloc::string::String;

pub use crate::{
    escape::{escape, escape_char, escape_into, Escape, PreEscaped},
    helpers::{from_fn, FromFn},
};

/// A type that can be represented in HTML.
///
/// Some types implementing this trait (`&str`, `char`) are escaped by default.
/// To render types unescaped, use [`PreEscaped`].
///
/// [`PreEscaped`]: crate::PreEscaped
pub trait IntoHtml {
    /// Writes the HTML into the given buffer.
    fn write_escaped(self, buf: &mut String);

    /// Allocates a new `String` with the given HTML.
    #[inline]
    fn into_string(self) -> String
    where
        Self: Sized,
    {
        let mut buf = String::new();
        self.write_escaped(&mut buf);
        buf
    }
}

macro_rules! via_itoa {
    ($($ty:ty)*) => {
        $(
            impl $crate::IntoHtml for $ty {
                #[inline]
                fn write_escaped(self, buf: &mut String) {
                    buf.push_str(itoa::Buffer::new().format(self))
                }
            }
        )*
    };
}

macro_rules! via_ryu {
    ($($ty:ty)*) => {
        $(
            impl $crate::IntoHtml for $ty {
                #[inline]
                fn write_escaped(self, buf: &mut String) {
                    buf.push_str(ryu::Buffer::new().format(self));
                }
            }
        )*
    };
}

via_itoa! {
    isize i8 i16 i32 i64 i128
    usize u8 u16 u32 u64 u128
}

via_ryu! { f32 f64 }

impl IntoHtml for &str {
    #[inline]
    fn write_escaped(self, buf: &mut String) {
        escape_into(buf, self)
    }
}

impl IntoHtml for String {
    #[inline]
    fn write_escaped(self, buf: &mut String) {
        self.as_str().write_escaped(buf)
    }
}

impl IntoHtml for char {
    #[inline]
    fn write_escaped(self, buf: &mut String) {
        escape_into(buf, self.encode_utf8(&mut [0; 4]));
    }
}

impl IntoHtml for bool {
    #[inline]
    fn write_escaped(self, buf: &mut String) {
        buf.push_str(if self { "true" } else { "false" })
    }
}

impl<T: IntoHtml> IntoHtml for Option<T> {
    #[inline]
    fn write_escaped(self, buf: &mut String) {
        if let Some(x) = self {
            x.write_escaped(buf)
        }
    }
}

macro_rules! impl_tuple {
	((
		$($i:ident,)+
	)) => {
		impl<$($i,)+> IntoHtml for ($($i,)+)
		where
			$($i: IntoHtml,)+
		{
			fn write_escaped(self, buf: &mut String) {
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

#[cfg(feature = "either")]
impl<L: IntoHtml, R: IntoHtml> IntoHtml for either::Either<L, R> {
    #[inline]
    fn write_escaped(self, buf: &mut String) {
        either::for_both!(self, x => x.write_escaped(buf))
    }
}
