macro_rules! via_itoap {
    ($($ty:ty)*) => {
        $(
            impl $crate::Render for $ty {
                #[inline]
                fn render_to(self, buf: &mut String) {
                    itoap::write_to_string(buf, self)
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
