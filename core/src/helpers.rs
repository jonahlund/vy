use alloc::string::String;

use crate::IntoHtml;

pub struct SizeHint(pub usize);

impl IntoHtml for SizeHint {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, _: &mut String) {}

    #[inline]
    fn size_hint(&self) -> usize {
        self.0
    }
}

pub struct FromFn<F: Fn(&mut String)>(pub F);

impl<F: Fn(&mut String)> IntoHtml for FromFn<F> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        self.0(buf)
    }

    #[inline]
    fn size_hint(&self) -> usize {
        0
    }
}
