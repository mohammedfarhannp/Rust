#[allow(unused_must_use)]

fn main() {
    let x: u32 = 5 as u32;
    
    let y: u32 = {
        let x_square = x * x;
        let x_cube = x_square * x;
        
        // This final statement of the code block returns the computed value to variable y
        // No semicolons must be used for final statement that returns value
        x + x_square + x_cube
    };
    
    let z = {
        // Let's see what happens when we use semicolon;
        x * x * x;
    };
    
    println!("x = {:?}", x);
    println!("y = {:?}", y);
    println!("z = {:?}", z);
}
