# Rust
Rust Programming Langauge (Me attempting to learn)

YouTube Video: https://youtu.be/BpPEoZW5IiY?t=798

# Notes
1. `fn` to define function
2. `println!()` or `print!()` to print to output terminal
3. Variable are assigned using `let` keyword
4. `i32` is for integer type.
5. print functions can't handle any other datatype apart from string literal, any other type must be formatted using `"{}"` example would be `println!("x is {}", x);`
6. Default is that a variable is immutable and to make it mutable, must use `mut` keyword while declaring variable like this `let mut x: i8 = 20;`
7. Variable scopes are similar to python language.
8. Can create code blocks without anything, don't know if it exists in C.
example
```rust
fn main()
{
    print!("Hello");
    {
        println!(" World!");
    }
}
```