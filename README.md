# vy

`vy` is a fast and convenient HTML templating library for Rust.

## Usage

Here is an example that shows how to create a typical HTML page:

``` rust
use vy::*;

fn page(content: impl ToHtml) -> impl ToHtml {
    (
        DOCTYPE,
        html!(
            head!(
                meta!(charset = "UTF-8"),
                title!("My Title"),
                meta!(
                    name = "viewport",
                    content = "width=device-width,initial-scale=1"
                ),
                meta!(name = "description", content = ""),
                link!(rel = "icon", href = "favicon.ico")
            ),
            body!(h1!("My Heading"), content)
        ),
    )
}
```

A few things noteworthy things you might've noticed:

- **All tags are macros.** There is no HTML-tree parsing (although there is a level of speculative parsing to improve performance).
- **Attributes are written inside the macro body.** This is rather unique to this crate, but is necessary since each tag is a self-contained macro invocation.
- **No container macro.** As hinted above, you do not have to wrap your HTML inside a container macro, like `html!(..)`. In fact, the [`html!`] macro will simply return the literal `<html>` tag.

## Syntax

The full grammar is rather simple:

``` text
spec := [attr],* [expr],*

attr := attr_name['?'] '=' expr
attr_name := identifier | text
```

The syntax is the same for all tags, although some tags have certain restrictions, such as void tags, which may only contain attributes.

Why this particular syntax was chosen isn't really a coincidence, rather, it was chosen to conform with Rust's syntax parser. In other words, tools like `rustfmt` will actually format the code inside macro bodies (assuming you are using parentheses `div!()` and not curly brackets `div!{}`). This is also the reason why certain attribute keywords are disallowed, such as `type` or `for`.

## Performance

The macros produce a tuple type implementing [`ToHtml`], which do not involve closures (nor eager allocations). This in turn means that the total size of the HTML can be estimated upfront, thus minimizing reallocations. In fact, in most cases the written HTML will fit inside a single allocation.
