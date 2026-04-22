fn main() {
    let x: i32 = 500; // Variable Initialized and used // Spits warning if not used
    let y: i32; // Variable Initialized but not used // Spits Warning
    let _z: i32; // Underscore at the beginning does not spit warning for variable not being used

    // By Default Variables are immutable
    let mut a: i8 = 3; // So use `mut` keyword to make the variable mutable
    
    a += 1;

    println!("The Value of x is {}", a);
}