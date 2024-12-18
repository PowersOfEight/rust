use core::panic;
use std::{io::{self, Write}, time::Instant};
use colored::*;

fn fib_recursive(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}

fn fib_tail_recursive(n: u32) -> u32 {
    fib_tail(n, 0, 1)
}

fn fib_tail(n: u32, current: u32, next: u32) -> u32 {
    match n {
        0 => current,
        _ => fib_tail(n-1, next, current + next)
    }
}

fn get_unsigned_input() -> u32 {
    loop {
        print!("Enter a non-negative integer: ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(e) => panic!("{}",e)
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input){
            Ok(x) =>println!("Success: {} characters read into input => \"{}\"", x.to_string().green(), input.trim().blue().on_yellow()),
            Err(x) => {
                println!("{}:{}", "Error".red().bold().underline(), x.to_string().yellow());
                continue;
            }
        }
        match input.trim().parse::<u32>() {
            Ok(n) => return n,
            Err(err) => {
                println!("{}:{}", "Error".red().bold().underline(), err.to_string().yellow());
                continue;
            }
        }
    }
}

fn border(letter: char, width: u8) {
    for _ in 0..width {
        print!("{letter}");
    }
    println!("");
}

fn time_execution<F>(label: &str, f: F)
where
    F: FnOnce() -> u32,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{} result: {}", label, result);
    println!("Time taken: {:?}", duration);
}

fn main() {
    loop {
        border('-', 80);
        println!("Choose an option:");
        border('-', 80);
        println!("1. Fibonacci (Recursive)");
        println!("2. Fibonacci (Tail Recursive)");
        println!("3. Compare both implementations");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("I never learned to read");
        let choice = match choice.trim().parse::<u8>() {
            Ok(x) => x,
            Err(y) => {
                println!("{}:{}", "ERROR: ".red().bold(), y);
                continue;
            }
        };

        match choice {
            1 | 2 | 3 => {
                let n = get_unsigned_input();
                match choice {
                    1 => {
                        time_execution("Recursive Fibonacci",|| fib_recursive(n));
                    },
                    2 => {
                        time_execution("Tail Recursive Fibonacci",|| fib_tail_recursive(n));
                    },
                    3 => {
                        println!("\nTiming Recursive Fibonacci:");
                        time_execution("Recursive Fibonacci", || fib_recursive(n));
                        println!("\nTiming Tail Recursive Fibonacci:");
                        time_execution("Tail Recursive Fibonacci", || fib_tail_recursive(n));
                    },
                    _ => {}
                }
            },
            4 => {
                println!("Exiting program! Goodbye...");
                break;
            },
            _ => {
                println!("If you keep choosing wrong...");
                println!("...You're gunna have a bad time...");
            }

        }

    }

}
