use vy_core::IntoHtml;

pub struct Attribute<K: IntoHtml, V: IntoHtml>(pub K, pub V);

impl<K: IntoHtml, V: IntoHtml> IntoHtml for Attribute<K, V> {
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self::Into {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        buf.push(' ');
        self.0.into_html().escape_and_write(buf);
        buf.push_str("=\"");
        self.1.into_html().escape_and_write(buf);
        buf.push('"');
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.size_hint() + self.1.size_hint() + 4
    }
}

pub struct OptionalAttribute<K: IntoHtml, V: IntoHtml>(pub K, pub V);

impl<K: IntoHtml> IntoHtml for OptionalAttribute<K, bool> {
    type Into = Option<Attribute<K::Into, ()>>;

    #[inline]
    fn into_html(self) -> Self::Into {
        self.1.then_some(Attribute(self.0.into_html(), ()))
    }

    #[inline]
    fn size_hint(&self) -> usize {
        if self.1 {
            self.0.size_hint() + 1
        } else {
            0
        }
    }
}

impl<K: IntoHtml, V: IntoHtml> IntoHtml for OptionalAttribute<K, Option<V>> {
    type Into = Option<Attribute<K::Into, V::Into>>;

    #[inline]
    fn into_html(self) -> Self::Into {
        self.1.map(|v| Attribute(self.0.into_html(), v.into_html()))
    }

    #[inline]
    fn size_hint(&self) -> usize {
        if let Some(x) = &self.1 {
            self.0.size_hint() + x.size_hint() + 4
        } else {
            0
        }
    }
}

pub struct SpreadAttributes<T: IntoHtml>(pub T);

impl<const N: usize, K: IntoHtml, V: IntoHtml> IntoHtml
    for SpreadAttributes<[Attribute<K, V>; N]>
{
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self::Into {
        self
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.size_hint()
    }
}

impl<K: IntoHtml, V: IntoHtml> IntoHtml
    for SpreadAttributes<Vec<Attribute<K, V>>>
{
    type Into = Self;

    #[inline]
    fn into_html(self) -> Self::Into {
        self
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.size_hint()
    }
}

macro_rules! impl_either {
    ( $( $name:ident $($var:ident)+, )+ ) => {
        $(
            #[doc(hidden)]
            pub enum $name<$($var),+> {
                $($var($var)),+
            }

            impl<$($var),+> IntoHtml for $name<$($var),+>
            where
                $($var: IntoHtml),+
            {
                type Into = $name<$($var::Into),+>;

                #[inline]
                fn into_html(self) -> Self::Into {
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

impl_either! {
    Either A B,
    Either3 A B C,
    Either4 A B C D,
    Either5 A B C D E,
    Either6 A B C D E F,
    Either7 A B C D E F G,
    Either8 A B C D E F G H,
    Either9 A B C D E F G H I,
}
