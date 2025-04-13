#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod escape;
pub mod helpers;

use alloc::string::String;

use self::escape::escape_into;

/// A type that can be represented as HTML.
pub trait IntoHtml {
    /// Converts this value into HTML by producing a type that implements
    /// [`IntoHtml`].
    ///
    /// This method enables composition of HTML structures by delegating
    /// rendering to the returned value. Use it to build nested HTML
    /// elements, combine components, or leverage existing [`IntoHtml`]
    /// implementations.
    ///
    /// # Examples
    ///
    /// Compose nested HTML elements using macros:
    ///
    /// ```
    /// # use vy::*;
    /// struct Article {
    ///     title: String,
    ///     content: String,
    ///     author: String,
    /// }
    ///
    /// impl IntoHtml for Article {
    ///     fn into_html(self) -> impl IntoHtml {
    ///         article!(
    ///             h1!(&self.title),
    ///             p!(class = "content", &self.content),
    ///             footer!("Written by ", &self.author)
    ///         )
    ///     }
    /// }
    /// ```
    ///
    /// Chain multiple implementations through delegation:
    ///
    /// ```
    /// # use vy::*;
    /// # struct Article;
    /// # impl IntoHtml for Article {
    /// #     fn into_html(self) -> impl IntoHtml {}
    /// # }
    /// struct ArticlePage {
    ///     title: String,
    ///     articles: Vec<Article>,
    /// }
    ///
    /// impl IntoHtml for ArticlePage {
    ///     fn into_html(self) -> impl IntoHtml {
    ///         html!(head!(title!(&self.title)), body!(&self.articles))
    ///     }
    /// }
    /// ```
    ///
    /// For "leaf" types (elements that render directly without children, like
    /// primitive values), **always return `self`** to avoid infinite recursion:
    ///
    /// ```
    /// # use vy::*;
    /// struct TextNode(String);
    ///
    /// impl IntoHtml for TextNode {
    ///     fn into_html(self) -> impl IntoHtml {
    ///         // Leaf type returns itself to terminate the rendering chain
    ///         self
    ///     }
    ///
    ///     fn escape_and_write(self, buf: &mut String) {
    ///         escape_into(buf, &self.0);
    ///     }
    ///
    ///     fn size_hint(&self) -> usize {
    ///         self.0.len()
    ///     }
    /// }
    /// ```
    ///
    /// Returning any other value here can cause infinite recursion when the
    /// default [`IntoHtml`] methods call `into_html()` again.
    fn into_html(self) -> impl IntoHtml;

    /// Writes the HTML into the `String`.
    #[inline]
    fn escape_and_write(self, buf: &mut String)
    where
        Self: Sized,
    {
        self.into_html().escape_and_write(buf);
    }

    /// Returns an estimated size of the HTML.
    #[inline]
    fn size_hint(&self) -> usize {
        0
    }

    /// Allocates a new `String` and writes the HTML into it.
    fn into_string(self) -> String
    where
        Self: Sized,
    {
        let html = self.into_html();
        let size = html.size_hint();
        let mut buf = String::with_capacity(size + (size / 10));
        html.escape_and_write(&mut buf);
        buf
    }
}

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
                    buf.push_str(itoa::Buffer::new().format(self))
                }

                #[inline]
                fn size_hint(&self) -> usize {
                    2
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

                #[inline]
                fn size_hint(&self) -> usize {
                    4
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
    fn into_html(self) -> impl IntoHtml {
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
    #[inline]
    fn into_html(self) -> impl IntoHtml {
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

impl IntoHtml for String {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        escape_into(buf, &self)
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl IntoHtml for bool {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        if self {
            "true"
        } else {
            "false"
        }
    }
}

impl<T: IntoHtml> IntoHtml for Option<T> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
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

macro_rules! impl_enum {
    ( $( $name:ident $($var:ident)+, )+ ) => {
        $(
            pub enum $name<$($var),+> {
                $($var($var)),+
            }

            impl<$($var),+> IntoHtml for $name<$($var),+>
            where
                $($var: IntoHtml),+
            {
                #[inline]
                fn into_html(self) -> impl IntoHtml {
                    match self {
                        $( $name::$var(value) => $name::$var(value.into_html()), )*
                    }
                }

                #[inline]
                fn escape_and_write(self, buf: &mut String) {
                    match self {
                        $( $name::$var(value) => value.escape_and_write(buf), )*
                    }
                }

                #[inline]
                fn size_hint(&self) -> usize {
                    match self {
                        $( $name::$var(value) => value.size_hint(), )*
                    }
                }
            }
        )*
    };
}

impl_enum! {
    Either A B,
    Either3 A B C,
    Either4 A B C D,
    Either5 A B C D E,
    Either6 A B C D E F,
    Either7 A B C D E F G,
    Either8 A B C D E F G H,
    Either9 A B C D E F G H I,
    Either10 A B C D E F G H I J,
    Either11 A B C D E F G H I J K,
    Either12 A B C D E F G H I J K L,
    Either13 A B C D E F G H I J K L M,
}

impl<B: IntoHtml, I: Iterator, F> IntoHtml for core::iter::Map<I, F>
where
    F: FnMut(I::Item) -> B,
{
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        for x in self {
            x.escape_and_write(buf);
        }
    }
}
