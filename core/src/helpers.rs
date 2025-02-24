use alloc::string::String;

use crate::IntoHtml;

pub struct FromFn<F>(F);

impl<F: FnOnce(&mut String)> IntoHtml for FromFn<F> {
    #[inline]
    fn write_escaped(self, buf: &mut String) {
        (self.0)(buf);
    }
}

#[inline]
pub fn from_fn<F: FnOnce(&mut String)>(f: F) -> FromFn<F> {
    FromFn(f)
}
