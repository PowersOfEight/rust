//! # add_one
//! A Library because it's a library.
//! I'm just trying to be cool.
//! 
use rand;
/// Adds one to the number given
/// and returns the result.
/// 
/// # Examples
/// 
/// ```
/// use add_one::add_one;
/// let input = 5;
/// let result = add_one(input);
/// assert_eq!(6, result);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add_one() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
