# Rust Language Notes — Part 1

> Personal learning notes for the Rust programming language.  
> Reference: [Rust Programming Tutorial](https://youtu.be/BpPEoZW5IiY?t=6564)

---

## Table of Contents

- [Basics](#basics)
- [Variables & Mutability](#variables--mutability)
- [Scoping & Shadowing](#scoping--shadowing)
- [Data Types](#data-types)
- [Functions & Macros](#functions--macros)
- [Compiler Hints & Attributes](#compiler-hints--attributes)
- [Ownership](#ownership)
- [Integer Types Reference](#integer-types-reference)

---

## Basics

- **Functions** are defined with the `fn` keyword.
- **Macros** are invoked with `!` (e.g., `println!`, `print!`). They generate code at compile time at the call site.
- Print macros only accept string literals directly; other types must be interpolated using `{}` placeholders.

```rust
let x = 42;
println!("x is {}", x);
```

---

## Variables & Mutability

- Variables are declared using `let`.
- All variables are **immutable by default**. Use `mut` to make them mutable.

```rust
let x: i32 = 10;       // immutable
let mut y: i8 = 20;    // mutable
```

- Prefixing an unused variable with an underscore (`_x`) suppresses compiler warnings.

---

## Scoping & Shadowing

- Variable scopes behave similarly to Python.
- Arbitrary code blocks can be introduced without any special construct.

```rust
fn main() {
    print!("Hello");
    {
        println!(" World!");
    }
}
```

- **Shadowing** allows redeclaring a variable with the same name, effectively creating a new binding.

---

## Data Types

| Type     | Default    | Size / Notes                              |
|----------|------------|-------------------------------------------|
| Integer  | `i32`      | Signed 32-bit by default                  |
| Float    | `f64`      | Double-precision by default               |
| Char     | 4 bytes    | Unicode scalar value (unlike C's 1 byte)  |
| Bool     | 1 byte     | `true` or `false`                         |
| Unit     | 0 bytes    | Empty tuple `()`                           |
| String   | `&str`     | String slice annotation                    |

- **Double quotes** for string literals, **single quotes** for `char` literals.
- Heap-allocated strings: `String::from("Hello")` — the stack variable holds a pointer to heap data, suitable for dynamically sized text.

---

## Functions & Macros

- Only macros use `!()` syntax; regular functions do not.
- `println!` and `print!` are macros that expand at compile time.
- Use `unimplemented!()` or `todo!()` as placeholders for functions not yet implemented (analogous to Python's `pass`).

---

## Compiler Hints & Attributes

Place these attributes before `fn main()` or above individual functions:

| Attribute                        | Purpose                                       |
|----------------------------------|-----------------------------------------------|
| `#[allow(unused_variables)]`     | Suppress warnings for unused variables        |
| `#[allow(unused_assignments)]`   | Suppress warnings for unused assignments      |
| `#[allow(dead_code)]`            | Suppress warnings for unused functions/code   |

---

## Ownership

> Ownership is a unique, central concept in Rust.

**The Three Rules of Ownership:**

1. Each value in Rust has an owner.
2. There can only be **one owner** at a time.
3. When the owner goes out of scope, the value is **dropped** (memory freed).

---

## Integer Types Reference

| Length   | Signed | Unsigned |
|----------|--------|----------|
| 8-bit    | `i8`   | `u8`     |
| 16-bit   | `i16`  | `u16`    |
| 32-bit   | `i32`  | `u32`    |
| 64-bit   | `i64`  | `u64`    |
| 128-bit  | `i128` | `u128`   |
| arch     | `isize`| `usize`  |

---

## Notes

- Rust does not have classes or traditional OOP, but OOP-style patterns can be achieved through **structs** and **traits**.

