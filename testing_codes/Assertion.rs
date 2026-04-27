fn main() {
    let x: i32 = 500; // Variable Initialized and used
    let y: i32 = 200;
    let z: i32 = 300; 
    
    assert_eq!((z+y), x, "Fail Message"); // Checkes if two values are equal and if not then it panics and prints error message, exits program.
    
    assert_ne!((z+y), x-1, "Fail Message"); // Similar to above function except it looks for unequal.
    
}