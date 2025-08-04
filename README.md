# vy

[![crates.io](https://img.shields.io/crates/v/vy.svg)](https://crates.io/crates/vy)
[![docs.rs](https://docs.rs/vy/badge.svg)](https://docs.rs/vy)
![build](https://github.com/jonahlund/vy/actions/workflows/ci.yml/badge.svg)
![license: MIT](https://img.shields.io/crates/l/vy.svg)

A convenient, type-safe HTML templating library for Rust

## Usage

Create a typical HTML page:

```rust
use vy::prelude::*;

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

## Performance

`vy` utilizes a few practices for fast rendering times:

- **Pre-calculated sizing**: HTML output size is estimated before allocation.
- **Single-allocation rendering**: Most templates render in one memory allocation.
- **Zero-cost composition**: Macros expand to tuple-based [`IntoHtml`] types without closures.
