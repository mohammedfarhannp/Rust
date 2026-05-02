use std::io;
use std::io::Write;

fn main() {
    let str_1 : String = str_input("Enter a String: ");
    is_palindrome(String);
}

fn str_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Flush Failure!");

    let mut input : String = String::new();

    io::stdin().read_line(&mut input).expect("Read Failure!");
    input.trim().to_string()
}

fn is_palindrome(string : &str) -> bool {

    let mut rev : String = String::new();
    let mut length : u8 = string.chars().count()

    for i in 0..length {
        
    }

    println!("{}", rev);
    false
}