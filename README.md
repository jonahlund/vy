# vy

[![crates.io](https://img.shields.io/crates/v/vy.svg)](https://crates.io/crates/vy)
[![docs.rs](https://docs.rs/vy/badge.svg)](https://docs.rs/vy)
![build](https://github.com/jonahlund/vy/actions/workflows/ci.yml/badge.svg)
![license: MIT](https://img.shields.io/crates/l/vy.svg)

Fast and minimal HTML templating macros in Rust.

## Usage

```rust
vy::render! {
  <div>
    <h1>"This is awesome!"</h1>
    <img src="/sunrise.png" />
  </div>
}
```

## Syntax

The parsing is done via [`tiny-rsx`](https://docs.rs/tiny-rsx), with a very simple and minimal JSX-like syntax.

- Rust expressions are interpreted with braces, e.g. `<span>{1 + 2}</span>`.
- Text should always be quoted, e.g. `<p>"My text"</p>`.
- Void tags should end with a forward slash, e.g. `<br />`.

## Escaping

Escaping is opt-out, meaning values are escaped automatically unless you use `PreEscaped(..)`.
