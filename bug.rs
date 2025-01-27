fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y += 1; // Modifying x through y is allowed
    println!("x = {}", x); // Prints x = 6

    // Trying to use z while y exists results in compile-time error:
    // println!("x = {}", *z);  // ERROR: cannot borrow `x` as immutable because it is also borrowed as mutable
}