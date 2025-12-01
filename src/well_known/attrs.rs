pub trait GlobalAttributes {}

#[cfg(feature = "well-known-alpinejs")]
#[allow(non_upper_case_globals)]
pub trait AlpineJsAttributes {
    const x_data: () = ();
    const x_text: () = ();
    const x_html: () = ();
    const x_model: () = ();
    const x_show: () = ();
    const x_transition: () = ();
    const x_for: () = ();
    const x_if: () = ();
    const x_init: () = ();
    const x_effect: () = ();
    const x_ref: () = ();
    const x_cloak: () = ();
    const x_ignore: () = ();
}

#[cfg(feature = "well-known-htmx")]
#[allow(non_upper_case_globals)]
pub trait HtmxAttributes {}
