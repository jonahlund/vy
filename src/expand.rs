#[macro_export]
macro_rules! x_literal {
    ($($arg:expr),*) => {
        $crate::PreEscaped($crate::concat!($(
            $crate::x_literal!(@ $arg)
        ),*))
    };
    (@ $arg:literal) => {
        $arg
    };
    (@ $arg:expr) => {
        $crate::Escape($arg)
    };
}

#[macro_export]
macro_rules! x_closure {
    ($($arg:expr),*) => {
        $crate::from_fn(|output| {
            const SIZE_HINT: usize = stringify!($($arg)*).len();
            output.reserve(SIZE_HINT);
            $(
                $crate::x_closure!(@ output, $arg);
            )*
        })
    };
    (@ $buf:expr, $arg:literal) => {
        $buf.push_str($arg)
    };
    (@ $buf:expr, $arg:expr) => {
        $crate::ToHtml::write_escaped(&$arg, $buf)
    };
}

#[macro_export]
macro_rules! x_string {
    ($($arg:expr),*) => {{
        const SIZE_HINT: usize = stringify!($($arg)*).len();
        let mut output = String::with_capacity(SIZE_HINT);
        $(
            $crate::x_string!(@ &mut output, $arg);
        )*
        output
    }};
    (@ $buf:expr, $arg:literal) => {
        $buf.push_str($arg)
    };
    (@ $buf:expr, $arg:expr) => {
        $crate::ToHtml::write_escaped(&$arg, $buf)
    };
}
