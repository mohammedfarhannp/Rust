use std::io;
use std::io::Write;

fn main() {
    let x : u32 = input_u32("Enter a number to find all prime between 1 and your number: ");
    find_prime(x);
}

fn find_prime(num : u32) -> () {
    for i in 1..num {
        if is_prime(i)
        {
            print!("{} ", i);
        }
    }
}

fn is_prime(n : u32) -> bool {
    if n == 1 {
        return false;
    } else if n == 2 {
        return true;
    } else if n % 2 == 0 {
        return false;
    } else {
        for i in 3..n {
            if n % i == 0 {
                return false;
            }
            if i * i >= n {
                break;
            }
        }
    }

    true
}

fn input_u32(prompt : &str) -> u32 {
    print!("{}", prompt);
    io::stdout().flush().expect("Flush Failure");

    let mut input : String = String::new();

    io::stdin().read_line(&mut input).expect("Input Failure");

    input.trim().parse().expect("Enter Valid u32 type integer!")
}