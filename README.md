# Mutable and Immutable Borrows Conflict in Rust

This repository demonstrates a common error in Rust: attempting to use an immutable reference (&) to a variable after creating a mutable reference (&mut) to the same variable.  This results in a compile-time error because Rust's borrow checker prevents data races.

The `bug.rs` file contains code that demonstrates this error. The `bugSolution.rs` file provides a solution that correctly manages mutable and immutable borrows.

This is a simple illustration of a crucial concept in Rust's ownership system, teaching the correct way to handle mutable references to avoid these conflicts.