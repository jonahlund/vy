macro_rules! declare_elements {
    ($($name:ident { $($attr:ident)* }),*) => {
        // $(
        // #[macro_export]
        // macro_rules! $name {
        //     ($($tt:tt)*) => {
        //         $crate::define_element!(stringify!($name), $($tt)*)
        //     };
        // }
        // )*
    };
}

declare_elements!(div { GlobalAttributes }, span {});

#[macro_export]
macro_rules! div {
    ($($tt:tt)*) => {
        $crate::define_element!("div", $($tt)*)
    };
}

#[macro_export]
macro_rules! input {
    ($($tt:tt)*) => {
        $crate::define_element!("input", $($tt)*)
    };
}

pub trait Element {
    const VOID: bool = false;
}

pub struct Attribute;

pub struct Div;

impl Element for Div {}

pub struct Input;

#[expect(non_upper_case_globals)]
pub trait GlobalAttributes {
    const class: Attribute = Attribute;
    const id: Attribute = Attribute;
}

impl<T: Element> GlobalAttributes for T {}

#[expect(non_upper_case_globals)]
pub trait AriaAttributes {
    const aria_label: Attribute = Attribute;
    const aria_hidden: Attribute = Attribute;
    const aria_disabled: Attribute = Attribute;
}

impl<T: Element> AriaAttributes for T {}

#[cfg(feature = "known-htmx")]
#[expect(non_upper_case_globals)]
pub trait HtmxAttributes {
    const hx_target: Attribute = Attribute;
    const hx_get: Attribute = Attribute;
    const hx_post: Attribute = Attribute;
}

#[cfg(feature = "known-htmx")]
impl<T: Element> HtmxAttributes for T {}
