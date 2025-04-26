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

### Key Design Choices

- **Parenthesis-based**: Works with `rustfmt` formatting constraints.
- **Reserved word handling**: Attributes like `type` and `for` use string syntax, e.g., `"type" = ".."` instead of `type = ".."`.
- **Optional attributes**: `?` marks optional attributes (e.g., `disabled? = Some("")`).

### Why This Syntax?

The macro design balances several constraints:
1. Compatibility with Rust's syntax tree.
2. `rustfmt` compatibility (requires parenthesis syntax, e.g., `div!()` instead of `div!{}`).
3. Natural HTML-like authoring experience.
4. Compile-time validation opportunities.

## Performance

The template system is optimized for efficiency:

- **Pre-calculated sizing**: HTML output size is estimated before allocation.
- **Single-allocation rendering**: Most documents render in one memory allocation.
- **Zero-cost composition**: Uses tuple-based [`IntoHtml`] trait without closures.
