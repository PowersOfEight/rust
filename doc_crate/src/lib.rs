//! # doc_crate
//! 
//! `doc_crate` is a collection of utilities to demonstrate how to document a Rust crate.
//! 
mod kinds;

pub use self::kinds::{PrimaryColor, SecondaryColor};


/// Adds two numbers together.
/// 
/// # Examples
/// 
/// ```
/// let five = 5;
/// let seven = 7;
/// 
/// let result = doc_crate::add(five, seven);
/// 
/// assert_eq!(12, result);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// 
/// let answer = doc_crate::add_one(arg);
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
