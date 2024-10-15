# vy

[<img alt="github" src="https://img.shields.io/badge/github-jonahlund/vy-8da0cb?style=for-the-badge&logo=github">](https://github.com/jonahlund/vy)
[<img alt="crates.io" src="https://img.shields.io/crates/v/vy.svg?style=for-the-badge&logo=rust">](https://crates.io/crates/vy)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-vy-66c2a5?style=for-the-badge&logo=docs.rs">](https://docs.rs/vy)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/jonahlund/vy/ci.yml?branch=main&style=for-the-badge">](https://github.com/jonahlund/vy/actions?query=branch%3Amain)

Fast and minimal HTML templating macros for Rust.

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
