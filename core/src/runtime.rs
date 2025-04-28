pub trait IntoHtml {
    fn into_html(self) -> impl IntoHtml;

    #[inline]
    fn escape_and_write(self, buf: &mut String)
    where
        Self: Sized,
    {
        self.into_html().escape_and_write(buf);
    }

    fn size_hint(&self) -> usize {
        0
    }
}

impl IntoHtml for &str {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        buf.push_str(self);
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }
}

pub struct PreEscaped<T>(T);

impl<T: AsRef<str>> IntoHtml for PreEscaped<T> {
    #[inline]
    fn into_html(self) -> impl IntoHtml {
        self
    }

    #[inline]
    fn escape_and_write(self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }

    #[inline]
    fn size_hint(&self) -> usize {
        self.0.as_ref().len()
    }
}
