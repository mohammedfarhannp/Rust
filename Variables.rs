#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let x: i32 = 500; // Variable Initialized and used // Spits warning if not used
    let y: i32; // Variable Initialized but not used // Spits Warning
    let _z: i32; // Underscore at the beginning does not spit warning for variable not being used

    // By Default Variables are immutable
    let mut a: i8 = 3; // So use `mut` keyword to make the variable mutable
    
    a += 1;

    // Destructuring a tuple data structure
    let i: i8;
    let j: i8;
    let k: i16;
    
    (i, j, k) = (10, 20, 200);

    println!("The Value of i, j, k are {}, {}, {} respectively", i, j, k);
}