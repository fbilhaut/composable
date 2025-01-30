# 🔗 composable

## 💬 Introduction

Simple Rust crate to ease composition of "functional traits" and/or functions and/or closures.

Functional traits are defined by the `Composable` trait and its `apply` method.

Two or more traits and/or functions can be composed through different means:

* the `Composable::compose(self, other)` method,
* the `compose(one, other)` function,
* the `composed![...]` macro.

## 🎓 Examples

This crate is mostly useful for composing structures with a state and a possibly complex behavior, while providing a functional-style interface. However, the following examples are intentionally kept simplistic to illustrate its core operating principle.

```cargo
[dependencies]
composable = "0.9.0"
```

```rust
use composable::*;

struct AddTo { addend: usize }

impl Composable<usize, usize> for AddTo {
    fn apply(&self, input: usize) -> Result<usize> { 
        Ok(input + self.addend) 
    }
}

fn demo() -> Result<()> {
    let increment = AddTo { addend: 1 };
    let square = |x: usize| Ok(x * x);
    let composition = compose(increment, square);
    let result = composition.apply(2)?;
    assert_eq!(result, 9);
    Ok(())
}
```

This crate is particularly effective for describing pipelines in an almost declarative way. The `composed!` macro and its variants are provided for this purpose. For example:

```rust
let composition = composed![
    AddTo { addend: 1 },    
    Multiply { factor: 3 },    
    |x: usize| Ok(x * x),
    Print::new()
    // ...
];

let x = composition.apply(42)?;
```

## 🔨 Variants

It is also possible to combine a composable of type `X->(Y, V1)` with another one of type `Y->(Z, V2)` into a new one of type `X->(Z, (V1, V2))`.
This operation is provided by `compose_t()` and the `composed_t!` macro.

For example:

```rust
use composable::*;

struct AddToMsg { addend: usize }

impl Composable<usize, (usize, String)> for AddToMsg {
    fn apply(&self, input: usize) -> Result<(usize, String)> { 
        Ok((input + self.addend, "hello".to_string())) 
    }
}

fn demo_t() -> Result<()> {
    let increment = AddToMsg { addend: 1 };
    let square = |x: usize| Ok(((x * x), "world".to_string()));
    let composition = compose_t(increment, square);
    let (result, (msg1, msg2)) = composition.apply(2)?;
    assert_eq!(result, 9);
    assert_eq!(msg1, "hello");
    assert_eq!(msg2, "world");
    Ok(())
}
```

The symmetric operation is provided by `compose_rt()` and `composed_rt!`.


## 👉 Related

This crate was originally created as a sub-module of [🌿 gline-rs](https://github.com/fbilhaut/gline-rs), and has been externalized for use in other projects.
