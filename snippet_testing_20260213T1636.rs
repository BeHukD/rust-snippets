// snippet_testing_20260213T1636.rs
// Topic: Unit testing in Rust
// Demonstrates how to write simple unit tests and run them with `cargo test`.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Another function to demonstrate testing private logic
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// Unit tests go in the same file, annotated with #[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(-2, 3), -6);
        assert_eq!(multiply(0, 100), 0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panic() {
        let v = vec![1, 2, 3];
        v[99]; // This will panic
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(!is_even(3));
    }
}

// Integration test example (in tests/integration_test.rs, if you create that file)
// This main function is not needed for tests; it's just to show the library functions.
fn main() {
    println!("Add: {} + {} = {}", 2, 3, add(2, 3));
    println!("Multiply: {} * {} = {}", 2, 3, multiply(2, 3));
}

/*
To run tests:

cargo test

To run a specific test:

cargo test test_add

To see test output:

cargo test -- --nocapture

Note: This snippet includes a main function and tests. `cargo test` compiles and runs tests
in the same binary. `cargo run` runs the main function.

*/
