#![no_std]

mod escape;

pub use escape::PreEscaped;

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::string::String;

use crate::escape::escape_into;

/// A type that can be represented as HTML.
pub trait IntoHtml: Sized {
    type Into: IntoHtml;

    fn into_html(self) -> Self::Into;

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        self.into_html().escape_and_write(buf);
    }

    #[inline]
    fn into_string(self) -> String {
        let inner = self.into_html();
        let size = inner.size_hint();
        let mut buf = String::with_capacity(size + (size / 10));
        inner.escape_and_write(&mut buf);
        buf
    }

    #[inline]
    fn size_hint(&self) -> usize {
        0
    }
}

impl IntoHtml for &str {
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        escape_into(buf, self)
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl IntoHtml for char {
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        escape_into(buf, self.encode_utf8(&mut [0; 4]));
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len_utf8()
    }
}

impl IntoHtml for bool {
    type Into = &'static str;

    #[inline]
    fn into_html(self) -> Self::Into {
        if self {
            "true"
        } else {
            "false"
        }
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.into_html().size_hint()
    }
}

impl IntoHtml for String {
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self::Into {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        self.as_str().escape_and_write(buf);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl<T: IntoHtml> IntoHtml for Option<T> {
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self::Into {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        buf.reserve(self.size_hint());
        if let Some(x) = self {
            x.escape_and_write(buf)
        }
    }

    #[inline]
    fn size_hint(&self) -> usize {
        if let Some(x) = self {
            x.size_hint()
        } else {
            0
        }
    }
}

impl<B: IntoHtml, I: ExactSizeIterator, F> IntoHtml for core::iter::Map<I, F>
where
    F: FnMut(I::Item) -> B,
{
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self::Into {
        self
    }

    #[inline]
    fn escape_and_write(mut self, buf: &mut String) {
        let len = self.len();
        let first = self.next();
        buf.reserve(len * first.size_hint());
        first.escape_and_write(buf);
        for x in self {
            x.escape_and_write(buf);
        }
    }
}

macro_rules! impl_tuple {
	( ( $($i:ident,)+ ) ) => {
		impl<$($i,)+> IntoHtml for ($($i,)+)
		where
			$($i: IntoHtml,)+
		{
		    #[allow(unused_parens)]
		    type Into = ($( $i::Into ),+ );

            #[inline]
            fn into_html(self) -> Self::Into {
				#[allow(non_snake_case)]
				let ($($i,)+) = self;
				($( $i.into_html() ),+)
            }

            #[inline]
			fn escape_and_write(self, buf: &mut String) {
			    buf.reserve(self.size_hint());
				#[allow(non_snake_case)]
				let ($($i,)+) = self;
				$(
					$i.escape_and_write(buf);
				)+
			}

            #[inline]
            fn size_hint(&self) -> usize {
				#[allow(non_snake_case)]
				let ($($i,)+) = self;
                let mut n = 0;
				$(
					n += $i.size_hint();
				)+
                n
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

impl_tuple!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);

macro_rules! via_itoa {
    ($($ty:ty)*) => {
        $(
            impl $crate::IntoHtml for $ty {
                type Into = Self;

                #[inline]
                fn into_html(self) -> Self::Into {
                    self
                }

                #[inline]
                fn escape_and_write(self, buf: &mut String) {
                    itoap::write_to_string(buf, self);
                }
            }
        )*
    };
}

macro_rules! via_ryu {
    ($($ty:ty)*) => {
        $(
            impl $crate::IntoHtml for $ty {
                type Into = Self;

                #[inline]
                fn into_html(self) -> Self::Into {
                    self
                }

                #[inline]
                fn escape_and_write(self, buf: &mut String) {
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

impl IntoHtml for () {
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self::Into {
        self
    }

    #[inline]
    fn escape_and_write(self, _: &mut String) {}

    #[inline]
    fn size_hint(&self) -> usize {
        0
    }
}

impl<T: IntoHtml> IntoHtml for alloc::vec::Vec<T> {
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self::Into {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        for x in self {
            x.escape_and_write(buf);
        }
    }

    #[inline]
    fn size_hint(&self) -> usize {
        let mut n = 0;
        for x in self {
            n += x.size_hint();
        }
        n
    }
}

impl<T: IntoHtml, const N: usize> IntoHtml for [T; N] {
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self::Into {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        for x in self {
            x.escape_and_write(buf);
        }
    }

    #[inline]
    fn size_hint(&self) -> usize {
        let mut n = 0;
        for x in self {
            n += x.size_hint();
        }
        n
    }
}
