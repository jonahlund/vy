# Vy

A fast, minimal and correct HTML templating library.

## Usage

The exposed api is designed to be minimal but flexible.

### Creating a constant

```rust
const _: vy::PreEscaped<&str> = vy::lit! {
  <h1>"Hello, World!"</h1>
};
```

### Reusing templates as macros

```rust
macro_rules! page {
  ($m:path, $title:expr, $body:expr) => {
    vy::forward!($m,
      <!DOCTYPE html>
      <html>
        <head>
          <title>{$title}</title>
        </head>
        <body>{$body}</body>
      </html>
    )
  }
}

// Use our template in a constant
const _: vy::PreEscaped<&str> = page!(vy::str, "My Website", vy::lit! {
  <h1>"Welcome!"</h1>
});
```
