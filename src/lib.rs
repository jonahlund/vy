fn t() {
    // vy::div!(id = "foo", class = "bar");

    {
        const _: () = {
            _ = vy::div!();
            _ = class;
        };
        (
            PreEscaped("<div id=\"foo\" class=\"bar\""),
            123,
            PreEscaped("</div>"),
        )
    }
}

trait Global {
    const CLASS: () = ();
}

trait Div {
    const FORSEN: () = ();
}
