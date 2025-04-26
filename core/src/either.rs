use crate::{Buffer, IntoHtml};

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
                fn escape_and_write(self, buf: &mut Buffer) {
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
}
