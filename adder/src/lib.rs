
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn greeting(name: &str) -> String {
    // String::from("Hello!")
    format!("Hello {name}!")
}

pub mod shapes;
pub mod guess;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_logger() {}
    #[test]
    fn test_the_database() {}
    #[test]
    fn test_logger_and_database() {}

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        use guess::Guess;
        Guess::new(200);
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"    
        );
    }
    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

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

    
    #[test]
    fn smaller_cannot_hold_larger() {
        use shapes::rectangle::Rectangle;

        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }


    #[test]
    #[should_panic(expected = "Make this test fail")]
    fn another() {
        panic!("Make this test fail");
    } 

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2,2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
