use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::num::IntErrorKind;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(x) => {
                println!("\x1b[1;31mInput Error: \x1b[33m\"{x}\"\x1b[0m");
                continue;
            }
        };

        println!("You guessed: \"{guess}\"");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
