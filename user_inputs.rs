use std::io; // Standard Input/Output module
use std::io::Write; // For flushing...

#[allow(unused_assignments)]
#[allow(unused_variables)]
fn main() {
    //let s: String = string_input("What's you name: ");
    //println!("Welcome back Mr. {}", s);    

    let x: i32 = input_i32("Enter a Number: ");
    println!("Out: {}", x);
    
    
}

// Similar for floats, chars, integers, bools etc.
fn input_i32(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush");
    
    let mut input = String::new(); // pointer string type
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    input.trim().parse().expect("Enter a valid integer")
    
}


#[allow(dead_code)]
fn string_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush");
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    // Remove only the trailing newline
    input.trim_end_matches('\n').to_string()
}