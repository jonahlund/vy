use std::collections::HashMap;

use vy_core::IntoHtml;

#[cfg(feature = "well-known")]
pub mod well_known;

struct OptionalAttribute();

struct SpreadAttributes<
    K: IntoHtml,
    V: IntoHtml,
    I: IntoIterator<Item = (K, V)>,
>(pub I);

/// Acts as a group for child elements.
///
/// This can be used in cases where a tuple may reach the maximum length or for
/// general performance increase.
#[macro_export]
macro_rules! _frag {
    ($($x:expr),*) => {{}};
}

#[macro_export]
macro_rules! _void_tag {
    ($name:literal) => {};
}

#[macro_export]
macro_rules! _tag {
    ($name:literal) => {
        const _: () = {

        }
    };
}

#[macro_export]
macro_rules! div {
    ($($x:expr),*) => {
        $crate::_tag!("div", $($x),*)
    };
}

#[macro_export]
macro_rules! input {
    ($($x:expr),*) => {
        $crate::_void_tag!("input", $($x),*)
    };
}

fn t() {
    let mut map = HashMap::new();
    map.insert("abc", "123");

    SpreadAttributes(map);
}
