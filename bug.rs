fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x
    *y += 1; // Modifies x through y
    *z += 1; // This will cause a data race because both y and z try to modify x concurrently
    println!("x = {}", x);
}