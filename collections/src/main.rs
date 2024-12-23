mod tutorial;
pub use crate::tutorial::{vectors, strings, hash};
pub use crate::tutorial::exercises::stats as stats;
use rand::Rng;
use vectors::vector_examples as vector_examples;
use strings::*;
use hash::hash_map_demo as hash;
use std::vec::Vec;

fn generate_random_vec(size: usize) -> Vec<i32> {
    (0..size).map(|_| rand::thread_rng().gen_range(1..=100)).collect()
}

mod test{
    use std::fmt::Debug;

    use colored::Colorize;
    #[derive(Debug, PartialEq)]
    pub enum TestResult<R> {
        Pass(R),
        Fail(R, String),
    }

    #[derive(Debug)]
    pub struct TestCase< T, R, F>{
        pub input: T,// The expected input type for the test
        pub expected: R,// The expected result type
        pub function: F
        // Anything that uses TestCase<T,R> should implement
        // its own way of testing. Also, note that the TestCase
        // owns these publicly.  We're not obfuscating anything here
    }

    impl<T,R,F> TestCase<T,R,F> 
    where
        F: Fn(&T) -> R,
        R: PartialEq + Debug, {
        pub fn create(input: T, expected: R, function: F) -> Self {
            Self {
                input,
                expected,
                function
            }
        }

        pub fn run(&self) -> TestResult<R> {
            use TestResult::*;
            let result = (self.function)(&self.input);
            if result == self.expected {
                Pass(result)
            } else {
                let mut fail_message = "Expected: ".yellow();
                fail_message = format!("{fail_message}{:?}", self.expected).green();
                fail_message = format!("{fail_message}; Got: ").yellow();
                fail_message = format!("{fail_message}{:?}", result).red();
                Fail(
                    result,
                    fail_message.to_string()
                )
            }
        }
    }
}


fn main() {
    use colored::Colorize;
    use test::{TestCase, TestResult};
    use stats::{median, mode};
    use TestResult::{Pass, Fail};
    vector_examples();
    string_demo();
    hash();
    let mut median_cases = Vec::new();
    median_cases.push(
        TestCase::create(
            vec![1,2,3], 
            Some(2.0), 
            median.clone()));
    median_cases.push(
        TestCase::create(
            vec![1,2,3,4], 
            Some(2.5), 
            median.clone()));
    median_cases.push(
        TestCase::create(
            vec![],
            None,
            median.clone()
        )
    );

    for case in median_cases {
        match case.run() {
            Pass(result) => {
                let congrats = format!("Test Passed: {:?} ", result).green();
                println!("{congrats}");
            },
            Fail(_, msg) => {
                println!("{}{msg}", "Failure => ".red());
            }
        }
    }

    let mut mode_cases = Vec::new();
    mode_cases.push(
        TestCase::create(
            vec![], 
            None,
        mode.clone())
    );
    mode_cases.push(
        TestCase::create(
            vec![1,1,2,2,2,3,3], 
            Some((2,3)), 
            mode.clone())
    );
    mode_cases.push(
        TestCase::create(
            vec![0,1,0,2,0,3,0,4], 
            Some((0, 4)), 
            mode.clone())
    );

    mode_cases.iter().for_each(|x| {
        match x.run() {
            Pass(r) => {
                let congrats = format!("Test Passed: {:?} => {:?}", x.input, r);
                println!("{}",congrats.green());
            },
            Fail(_, msg) => {
                println!("{}{msg}", "Failure => ".red());
            }
        }
    });
}
