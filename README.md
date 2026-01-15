# vy

## 1: Syntax

let a = div!();

compile_rsx!(
  <html lang? = Some("en")>
    <button>
    </button>
  </html>
)

compile_mac!(
  html!(
    lang? = Some("en"),
  )
)

## 2: Ast 

Element {
  
}

## 3: Shared representation

## Target

- string
- wasm
