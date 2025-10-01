use alloc::string::String;

use crate::IntoHtml;

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
			fn escape_and_write(self, buf: &mut String) {
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

impl_tuple!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z A_ B_ C_ D_ E_ F_ G_ H_ I_ J_ K_);

macro_rules! via_itoa {
    ($($ty:ty)*) => {
        $(
            impl $crate::IntoHtml for $ty {
                #[inline]
                fn into_html(self) -> impl IntoHtml {
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
                #[inline]
                fn into_html(self) -> impl IntoHtml {
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
