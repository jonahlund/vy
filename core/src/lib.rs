#![no_std]

mod escape;

pub use escape::PreEscaped;

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::string::String;

use crate::escape::escape_into;

/// A type that can be represented as HTML.
pub trait ToHtml {
    fn escape_and_write(&self, buf: &mut String);

    #[inline]
    fn to_string(&self) -> String {
        let size = self.size_hint();
        let mut buf = String::with_capacity(size + (size / 10));
        self.escape_and_write(&mut buf);
        buf
    }

    #[inline]
    fn size_hint(&self) -> usize {
        0
    }
}

impl ToHtml for &str {
    #[inline]
    fn escape_and_write(&self, buf: &mut String) {
        escape_into(buf, self)
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl ToHtml for char {
    #[inline]
    fn escape_and_write(&self, buf: &mut String) {
        escape_into(buf, self.encode_utf8(&mut [0; 4]));
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len_utf8()
    }
}

impl ToHtml for bool {
    #[inline]
    fn escape_and_write(&self, buf: &mut String) {
        if *self {
            buf.push_str("true");
        } else {
            buf.push_str("false");
        }
    }

    #[inline]
    fn size_hint(&self) -> usize {
        if *self {
            4
        } else {
            5
        }
    }
}

impl ToHtml for String {
    #[inline]
    fn escape_and_write(&self, buf: &mut String) {
        self.as_str().escape_and_write(buf);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl<T: ToHtml> ToHtml for Option<T> {
    #[inline]
    fn escape_and_write(&self, buf: &mut String) {
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

// impl<B: ToHtml, I: ExactSizeIterator, F> ToHtml for core::iter::Map<I, F>
// where
//     F: FnMut(I::Item) -> B,
// {
//     #[inline]
//     fn escape_and_write(&self, buf: &mut String) {
//         let len = self.len();
//         let first = self.next();
//         buf.reserve(len * first.size_hint());
//         first.escape_and_write(buf);
//         for mut x in self {
//             x.escape_and_write(buf);
//         }
//     }
// }

macro_rules! impl_tuple {
    () => {
        impl $crate::ToHtml for () {
            #[inline]
            fn escape_and_write(&self, _: &mut String) {}
        }
    };
    ($(($i:tt $T:ident))+) => {
        impl<$($T: ToHtml),*> ToHtml for ($($T,)*) {
            #[inline]
            fn escape_and_write(&self, buf: &mut String) {
                $(self.$i.escape_and_write(buf);)*
            }

            #[inline]
            fn size_hint(&self) -> usize {
                let mut n = 0;
                $(
                    n += self.$i.size_hint();
                )*
                n
            }
        }
    }
}

impl_tuple!();
impl_tuple!((0 T));
impl_tuple!((0 T0) (1 T1));
impl_tuple!((0 T0) (1 T1) (2 T2));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7) (8 T8));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7) (8 T8) (9 T9));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7) (8 T8) (9 T9) (10 T10));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7) (8 T8) (9 T9) (10 T10) (11 T11));

macro_rules! via_itoa {
    ($($ty:ty)*) => {
        $(
            impl $crate::ToHtml for $ty {
                #[inline]
                fn escape_and_write(&self, buf: &mut String) {
                    itoap::write_to_string(buf, *self);
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
                fn escape_and_write(&self, buf: &mut String) {
                    buf.push_str(ryu::Buffer::new().format(*self));
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

impl<T: ToHtml> ToHtml for alloc::vec::Vec<T> {
    #[inline]
    fn escape_and_write(&self, buf: &mut String) {
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

impl<T: ToHtml, const N: usize> ToHtml for [T; N] {
    #[inline]
    fn escape_and_write(&self, buf: &mut String) {
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
