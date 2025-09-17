# vy

[![crates.io](https://img.shields.io/crates/v/vy.svg)](https://crates.io/crates/vy)
[![docs.rs](https://docs.rs/vy/badge.svg)](https://docs.rs/vy)
![build](https://github.com/jonahlund/vy/actions/workflows/ci.yml/badge.svg)
![license: MIT](https://img.shields.io/crates/l/vy.svg)

A convenient, type-safe HTML templating library for Rust

## Usage

```rust
use vy::prelude::*;

// Create a typical HTML page
fn page(content: impl IntoHtml) -> impl IntoHtml {
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
            body!(
                h1!("My Heading"),
                content
            )
        ),
    )
}

// Here's how to express common patterns in the VY tuple-based syntax
fn patterns(items: Vec<String>, condition: bool, maybe_some: Option<String>) -> impl IntoHtml {
    (
        // for loop
        items.into_iter().map(|item| li!(item)),
        // if-then-else
        condition.then_some(i!("condition is met")),
        (!condition).then_some(b!("condition is NOT met")),
        // if-let
        maybe_some.map(|inner| i!(inner)),
    )
}

assert_eq!(
    patterns(vec!["foo".into(), "bar".into()], true, Some("some".into())).into_string(),
    "<li>foo</li><li>bar</li><i>condition is met</i><i>some</i>"
);

assert_eq!(
    patterns(vec![], false, None).into_string(),
    "<b>condition is NOT met</b>"
);
```

Key features to note:

- **Tag macros**: HTML elements are created using dedicated macros.
- **Inline attributes**: Attributes are declared directly within macro bodies using `key = value` syntax.
- **Zero wrapping**: No need for container macros – elements compose naturally.
- **Void element support**: Automatically handles self-closing tags like `<meta>`, `<img>`, etc.

## Syntax

The macro grammar follows this pattern:

```text
element := [attribute],* [content],*

content := expression
attribute := name['?'] '=' expression
name := identifier | text
```

### Key design choices

- **Parenthesis-based**: Works with `rustfmt` formatting constraints.
- **Reserved word handling**: Attributes like `type` and `for` use string syntax, e.g., `"type" = ".."` instead of `type = ".."`.
- **Optional attributes**: `?` marks optional attributes (e.g., `class? = Some("foo")` or `disabled? = true`).

### Why this syntax?

The macro design balances several constraints:

- Compatibility with Rust's syntax tree.
- `rustfmt` compatibility (requires parenthesis syntax, e.g., `div!()` instead of `div!{}`).
- Natural HTML-like authoring experience.
- Compile-time validation.

## Escaping

Escaping is done automatically, but can be opted out by wrapping a type with `PreEscaped(..)`.

## Deriving for newtypes

For newtype wrappers (structs with a single unnamed field), you can use the
`InnerIntoHtml` derive macro to automatically implement `IntoHtml`:

```rust
use vy::prelude::*;

#[derive(InnerIntoHtml)]
struct Length((usize, String));

let length = Length((1, "cm".into()));
assert_eq!(length.into_string(), "1cm");
```

This delegates the `IntoHtml` implementation to the inner type, reducing
boilerplate.


## Performance

`vy` utilizes a few practices for fast rendering times:

- **Pre-calculated sizing**: HTML output size is estimated before allocation.
- **Single-allocation rendering**: Most templates render in one memory allocation.
- **Zero-cost composition**: Macros expand to tuple-based [`IntoHtml`] types without closures.


## Contributing

You can run `./.pre-commit.sh` before sending a PR, it will check everything the CI does.
