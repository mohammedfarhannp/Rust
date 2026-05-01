use std::io;
use std::io::Write;

fn main() {
    let x : u32 = input_u32("Enter a u32 integer: ");

    if is_palindrome(x) {
        println!("{} is palindrome", x);
    } else {
        println!("{} is not palindrome", x);
    }
}

fn is_palindrome(num : u32) -> bool {
    let mut tmp = num;
    let mut rev = 0;

    while tmp != 0 {
        rev = rev * 10 + tmp % 10;
        tmp = tmp / 10;
    }
    
    if rev == num {
        return true;
    }
    false
}


fn input_u32(prompt : &str) -> u32 {
    print!("{}", prompt);
    io::stdout().flush().expect("Flush Failure");

    let mut input : String = String::new();

    io::stdin().read_line(&mut input).expect("Read Failure");

    input.trim().parse().expect("Invalid Type! Enter proper u32 integer type!!")
}