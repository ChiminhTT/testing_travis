#![deny(missing_docs)]
//! The `testing` crate provides functions that add numbers to other numbers.
//! 
//! # Examples
//!
//! ```
//! assert_eq!(4, testing::add_two(2));
//! ```


/// This function adds two to its arguments.
///
/// # Examples
///
/// ```
/// use testing::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}


#[cfg(test)]
mod tests {

    use super::add_two;

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn should_panic() {
        assert_eq!("Hello", "world")
    }

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
