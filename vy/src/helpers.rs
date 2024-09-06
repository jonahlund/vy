macro_rules! via_itoa {
    ($($ty:ty)*) => {
        $(
            impl $crate::Render for $ty {
                #[inline]
                fn render_to(self, buf: &mut String) {
                    buf.push_str(itoa::Buffer::new().format(self));
                }
            }
        )*
    };
}

macro_rules! via_ryu {
    ($($ty:ty)*) => {
        $(
            impl $crate::Render for $ty {
                #[inline]
                fn render_to(self, buf: &mut String) {
                    buf.push_str(ryu::Buffer::new().format(self));
                }
            }
        )*
    };
}
