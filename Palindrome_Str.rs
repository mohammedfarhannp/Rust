use std::io;
use std::io::Write;

fn main() {
    let str_1 : &str = str_input("Enter a String: ");
    if is_palindrome(str_1) {
        println!("{} is palindrome!", str_1);
    } else {
        println!("{} is not palindrome!", str_1);
    }
}

fn str_input(prompt: &str) -> &str {
    print!("{}", prompt);
    
}