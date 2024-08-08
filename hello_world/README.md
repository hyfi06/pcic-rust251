# Primeros pasos en Rust

## Hello world


## Tipos
```rust
fn main() {
    let n: i32 = 10;
    println!("{}", n);
}
```

```rust
fn main() {
    let n: u32 = 10;
    let m = 10; // inferencia de tipos
    println!("{}", n + m);
}
```

```rust
fn main() {
    let n = 10i64;
    let m = 10; // inferencia de tipos
    println!("{}", n + m);
}
```