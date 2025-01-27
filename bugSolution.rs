fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modify x through y
    println!("x = {}", x); // Prints x = 6

    // To use an immutable reference after modifying, drop the mutable one first:
    drop(y);
    let z = &x; 
    println!("x = {}", *z); // Prints x = 6. No error!
} 