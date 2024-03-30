# Simplex-Rust

Simplex-Rust (simplex_rust) is an experimental implementation of simplex algorithm. I'm using it in order to study
optimization and network flows.

## Adding variables

```rust
let x1 = Var::new("x1", 75.0f32, Binary);
```