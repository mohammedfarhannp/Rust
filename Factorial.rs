use std::io;
use std::io::Write;

fn main() {
    let x : u32 = input_u32("Enter a positive integer to find factorial: ");
    let out : u32 = fact(x);
    println!("Factorial of {} is {}", x, out);
}

fn input_u32(prompt : &str) -> u32 {
    print!("{}", prompt);
    io::stdout().flush().expect("Flush Failure");

    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("Read Failure");
    input.trim().parse().expect("Enter Valid u32 integer type!")
}

fn fact(n : u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        return n * fact(n - 1);
    }
}