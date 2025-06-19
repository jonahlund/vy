pub use vy_core::{
    escape::{escape, PreEscaped},
    IntoHtml,
};
pub use vy_macros::{define_element, define_void_element};

#[cfg(feature = "known-htmx")]
pub use crate::known::htmx::HtmxAttributes;
#[cfg(feature = "known-svg")]
pub use crate::known::svg;
pub use crate::{known::*, *};
