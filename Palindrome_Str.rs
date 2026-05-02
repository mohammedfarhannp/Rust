use std::io;
use std::io::Write;

fn main() {
    let str_1 : String = str_input("Enter a String: ");
    is_palindrome(str_1);
}

fn str_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Flush Failure!");

    let mut input : String = String::new();

    io::stdin().read_line(&mut input).expect("Read Failure!");
    input.trim().to_string()
}

fn is_palindrome(string : String) -> bool {

    let mut rev : String = String::new();
    let length : usize = string.chars().count();

    for i in 1..length+1 {
        rev.push(string.chars().nth(length - i).expect("FAAAAA"))
    }

    println!("{}", rev);
    false
}