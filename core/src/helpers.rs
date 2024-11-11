use crate::ToHtml;

pub struct FromFn<F: Fn(&mut String)>(F);

impl<F: Fn(&mut String)> ToHtml for FromFn<F> {
    #[inline]
    fn to_html(&self, buf: &mut String) {
        (self.0)(buf);
    }
}

#[inline]
pub fn from_fn<F: Fn(&mut String)>(f: F) -> impl ToHtml {
    FromFn(f)
}
