use std::io;
use std::io::Write;

fn main() {
    let x : u32 = input_u32("Enter a upper limit for Fibonacci Series: ");
    fib(x);
}

fn fib(i : u32) -> () {
    let mut first = 0;
    let mut second = 1;
    
    if i == 0
    {
        print!("{}",first);
    } else {
        print!("{} {} ", first, second);
        let mut third = first + second;
        while third < i  {
            print!("{} ", third);
            first = second;
            second = third;
            third = first + second;
        }
    }
}


fn input_u32(prompt: &str) -> u32 {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush");
    
    let mut input = String::new(); // pointer string type
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    input.trim().parse().expect("Enter a valid integer")    
}