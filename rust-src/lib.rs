// This file contains the public function for adding two numbers.

/// Adds two 32-bit signed integers together.
///
/// # Arguments
///
/// * `a` - The first integer.
/// * `b` - The second integer.
///
/// # Returns
///
/// The sum of `a` and `b`.
///
/// # Examples
///
/// ```
/// let sum = rust_adder::add_two_numbers(5, 3);
/// assert_eq!(sum, 8);
/// ```
pub fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// Unit tests are placed in the same file as the code they're testing
#[cfg(test)]
mod tests {
    // Import all items from the parent module (the library code above)
    use super::*;

    #[test]
    fn it_adds_correctly_KD_12() {
        // Call the function with two numbers.
        let result = add_two_numbers(10, 15);
        // Assert that the result is what we expect.
        assert_eq!(result, 25);
    }

    //I WANT THIS TEST TO REPORT AS FAILED
    #[test]
    fn it_handles_negative_numbers_KD_14() {
        let result = add_two_numbers(-5, 0);
        assert_eq!(result, 7);
    }

    #[test]
    fn it_handles_zero_KD_13() {
        let result = add_two_numbers(0, 42);
        assert_eq!(result, 42);
    }
}
