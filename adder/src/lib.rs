
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub mod shapes;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        use shapes::rectangle::Rectangle;

        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    // #[test]
    // fn exploration() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // } 
}
