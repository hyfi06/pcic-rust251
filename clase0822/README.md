# Clase 2024-08-22

(): unit type
!: never unit

Usa los tipos para hacer irreprecentables los estados inválidos

## formas de crear tipos

Tipos algebraicos

Producto -> struct

```rust
struct S {
  f1: f64,
  f2: i32,
}
```

suma -> enum

```rust
enum Direccion {
  Arriba,
  Abajo,
  Izq,
  Der,
}
```

```rust
impl Option<T>{
  fn unwrup(self){
    match self {
      some(v) => {v}
      none => {panic!c;}
    }
  }
}

let m:i32 = minimo(89[..]).unwrap()?;

```

```rust
todo!();
```

```rust
::<>
```
