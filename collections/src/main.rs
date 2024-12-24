mod tutorial;
pub use crate::tutorial::{vectors, strings, hash};
pub use crate::tutorial::exercises::{stats as stats, pig_latin as pig_latin};
// use rand::Rng;
use vectors::vector_examples as vector_examples;
use strings::*;
use hash::hash_map_demo as hash;
use std::vec::Vec;

// fn generate_random_vec(size: usize) -> Vec<i32> {
//     (0..size).map(|_| rand::thread_rng().gen_range(1..=100)).collect()
// }

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


mod menu{
use crate::tutorial::exercises::departments::Company;
use std::io::{stdin, stdout, Write};


    fn print_border(n: usize) {
        for _ in 0..n {
            print!("-");
        }
        println!("");
    }

    fn display_menu(opts: &[&str]) {
        print_border(80);
        for (i, opt) in opts.iter().enumerate() {
            println!("({i}) -> {opt}")
        }
        print_border(80);
        println!("Anything else quits the program");
        print!("Enter your choice: ");
        stdout().flush().unwrap();
    }

    fn print_departments(company: &Company) -> () {
        company
            .department_employees
            .iter()
            .for_each(
                |(dep,emps)| {
                    let offset = dep.len();
                    println!("{dep}:");
                    emps
                        .iter()
                        .for_each(
                          |emp| {
                            println!("{:width$}|__{}", " ", emp, width=offset);
                          }  
                        );
                }
            );
    }

    fn add_employee(company: &mut Company) -> () {
        use std::collections::hash_map::Entry;
        use Entry::{Occupied, Vacant};
        println!();
        print!("Enter the name of the department: ");
        stdout().flush().unwrap();
        let mut buf = String::new();
        if let Ok(_) = stdin().read_line(&mut buf) {
            let mut name = String::new();
            print!("Enter the name of the employee: ");
            stdout().flush().unwrap();
            if let Ok(_) = stdin().read_line(&mut name) {
                buf = buf.trim().to_string();
                name = name.trim().to_string();
                match company.department_employees.entry(buf) {
                    Occupied(mut occ) => {
                        occ.get_mut().push(name);
                    },
                    Vacant(vac) => {
                        vac.insert(vec![name]);
                    }
                }
            } else {
                panic!("Could not read line!");   
            }
        } else {
            panic!("Could not read line!");
        }
    }

    pub fn main() {
        let options = [
            "Print the departments",
            "Add an employee to the department",
        ];
        let mut company = Company::new();
        company.add("Engineering", "Sally");
        company.add("Sales", "Amir");
        'main: loop{
            display_menu(&options);
            let mut input = String::new();
            match stdin()
                .read_line(&mut input){
                    Ok(_) => {
                        match input.trim().parse::<u8>() {
                            Ok(x) => {
                                match x {
                                    0 => print_departments(&company),
                                    1 => add_employee(&mut company),
                                    _ => {
                                        println!("Goodbye....");
                                        break 'main;
                                    }
                                }
                            },
                            Err(err) => {
                                dbg!(err);
                                break 'main;
                            }
                        }
                    },
                    Err(_) => {

                        break 'main;
                    }
            }
        }
    }
}

fn main() {
    use colored::Colorize;
    use test::{TestCase, TestResult};
    use stats::{median, mode};
    use pig_latin::{convert, split_and_convert};
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

    let latin_case = 
        TestCase::create(
            String::from("This is Pig Latin"),
            String::from("his-Tay is-hay ig-Pay atin-Lay"),
            split_and_convert.clone()
        );
    match latin_case.run() {
        Pass(result) => {
            println!("Input: {} => {}", latin_case.input, result.green())
        },
        Fail(_, message) => println!("{}",message) 
    }

    let latin_word =
        TestCase::create(
            String::from("first"),
            Some(String::from("irst-fay")),
            convert.clone()
        );
    match latin_word.run() {
        Pass(result) => {
            println!("Input: {} => {:?}", latin_word.input.green(), result)
        },
        Fail(_, message) => println!("{}", message)
    }

    let latin_vowel = 
        TestCase::create(
            String::from("apple"),
            Some(String::from("apple-hay")),
            convert.clone()
        );
    match latin_vowel.run() {
        Pass(result) => {
            println!("Input: {} => {:?}", latin_vowel.input.green(), result)
        },
        Fail(_, message) => println!("{}", message)
    }

    let latin_none =
        TestCase::create(
            String::from(""), 
            None, 
            convert.clone());
    match latin_none.run() {
        Pass(result) => {
            println!("Input: \"{}\" => {:?}", latin_none.input.green(), result)
        },
        Fail(_, message) => println!("{}", message)
    }

    menu::main();

}
