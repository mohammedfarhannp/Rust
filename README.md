# Rust
Rust Programming Langauge (Me attempting to learn)

YouTube Video: https://youtu.be/BpPEoZW5IiY?t=1052

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
9. Only macros are called with `!()` not functions, and therefore print and println are macros.
10. macros generate code at the location during compile time.
11. `&str` is datatype annotation for string.
12. Shadowing allows you to create variables with same name.
13. `#[allow(unused_variables)]` use of this before main at top to avoid warnings on unused variables or just rename the variable to start with underscore like this `let _x: i8 = 20;`
14. 
