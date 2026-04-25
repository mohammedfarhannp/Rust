fn main() {
    let x: i8 = 20;
    let y: i8 = 30;
    
    println!("{}", add(x, y));
}

#[allow(dead_code)]
fn example() {
    let x: &str = "Hello";
    println!("{} World", x);
}

fn add(a: i8, b: i8) -> i8 {
    return a + b;
}