#[allow(unused_variables)]
#[allow(unused_assignments)]


fn main() {
    // And
    println!("T AND T = {}", true & true);
    println!("T AND F = {}", true & false);
    println!("F AND F = {}", false & false);
    
    // Or
    println!("T OR F = {}", true | false);
    println!("F OR F = {}", false | false);
    println!("T OR T = {}", true | true);
    
    // XOR
    println!("T XOR F = {}", true ^ false);
    println!("T XOR T = {}", true ^ true);
    println!("F XOR F = {}", false ^ false);
    
    // Bitwise Shift
    println!("Bitwise Left Shift 1 five times = {}", 1<<5); // 00000001 -> 00100000
    println!("Bitwise Right Shift 32 five times = {}", 32>>5); // 00100000 -> 00000001
}