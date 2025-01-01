# Data Race in Rust with Multiple Mutable References

This repository demonstrates a common error in Rust: creating multiple mutable references to the same data, leading to data races.  Data races result in undefined behavior and can produce unpredictable results.

The `bug.rs` file contains the code that demonstrates the data race. The `bugSolution.rs` file shows how to correct the issue using techniques like ownership and borrowing.