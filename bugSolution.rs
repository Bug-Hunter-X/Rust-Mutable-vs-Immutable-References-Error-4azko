fn main() {
    let mut x = 5;
    { //Use a new scope to limit the lifetime of the mutable reference
        let y = &mut x; // y is a mutable reference to x
        *y = 6; 
    }
    let z = &x; // z is an immutable reference to x
    println!("x = {}", x); // Prints: x = 6
}