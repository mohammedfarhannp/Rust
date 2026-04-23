// Concept of shadowing a variable 
// Allows you to create variables with same name
// Shadowing replaces previous value of the variable with new one.

fn main() {
    let x: i8 = 12;
    {
        let x = 20;
        println!("x = {}", x);
        assert_eq!(x, 20);
    }
    println!("x = {}", x);
    assert_eq!(x, 12);
    
    let x = "Hello";
    println!("x = {}", x)
}