use crate::{buffer::Buffer, IntoHtml};

macro_rules! impl_tuple {
	( ( $($i:ident,)+ ) ) => {
		impl<$($i,)+> IntoHtml for ($($i,)+)
		where
			$($i: IntoHtml,)+
		{
            #[inline]
            fn into_html(self) -> impl IntoHtml {
				#[allow(non_snake_case)]
				let ($($i,)+) = self;
				($(
					$i.into_html()
				),+)
            }

            #[inline]
			fn escape_and_write(self, buf: &mut Buffer) {
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
                #[inline]
                fn into_html(self) -> impl IntoHtml {
                    self
                }

                #[inline]
                fn escape_and_write(self, buf: &mut Buffer) {
                    // Implementation from: https://raw.githubusercontent.com/rust-sailfish/sailfish/47e281cd1c5d8c3299955f360595e4c37d1d111c/sailfish/src/runtime/render.rs

                    use itoap::Integer;

                    // SAFETY: `MAX_LEN < 40` and then does not overflows `isize::MAX`.
                    // Also `b.len()` should be always less than or equal to `isize::MAX`.
                    unsafe {
                        buf.reserve_small(Self::MAX_LEN);
                        let ptr = buf.as_mut_ptr().add(buf.len());

                        // SAFETY: `MAX_LEN` is always greater than zero, so
                        // `b.as_mut_ptr()` always point to valid block of memory
                        let l = itoap::write_to_ptr(ptr, self);
                        buf.advance(l);
                    }
                    debug_assert!(buf.len() <= buf.capacity());
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
                fn into_html(self) -> impl IntoHtml {
                    self
                }

                #[inline]
                fn escape_and_write(self, buf: &mut Buffer) {
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
