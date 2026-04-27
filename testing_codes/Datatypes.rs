#[allow(unused_assignments)]

use std::mem::size_of_val;

fn main() {
    // String Datatype
    let a: &str = "Hello World!";
    println!("{}", a);

    // Integer Datatypes
    let b: u8 = 8; // Unsigned 8 bit integer // u8, u16, u32, u64, u128
    let c: i8 = -3; // Signed 8 bit integer // i8, i16, i32, i64, i128

    // Float Datatypes
    let pi: f32 = 3.14; // Floating point // f32 and f64

    // Character Datatype
    let n: char = 'C'; // Character

    // Boolean Datatype
    let x: bool = true; // Boolean Value True
    let y: bool = false; // Boolean Value False

    // Unit Datatype
    let unit: () = (); // Empty // Unit type
    // Functions without a return function would return a unit type
    println!("{}", size_of_val(&unit)) // Size would be zero


}
