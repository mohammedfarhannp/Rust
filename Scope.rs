fn main() {
    let x: i32 = 500;
    let y: i8;
    
    {
        let z: i8 = 40; // Local Scope Var
        y = 25; // Global Scope Var
        println!("Value of Z is {} and Value of Y is {}", z, y);
    }
    
    println!("Value of X is {} and Value of Y is {}!", x, y);
}