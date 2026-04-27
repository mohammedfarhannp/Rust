fn main() {
    
    let s1 = String::from("Hello World");
    let s2 = s1.clone(); // Copys or clones everything s1 has.
    
    
    /* let s2 = s1 */
    // Can't do this to copy, here s1 is no longer usable as data in rust can only have one owner
    
    println!("String 1 = {}\nString 2 = {}", s1, s2);
}