fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x
    //This next line will cause a compile time error because z is immutable reference and x is mutable 
    *y = 6;
}