use std::thread;
use std::time::Duration;
use std::env::args;

fn main() {

    if let Some(name) = args().nth(0) {
        println!("Threading \"{name}\"");
        for letter in name.chars() {

            thread::spawn(move || {
                thread::sleep(Duration::from_millis(1));
                println!("{letter}");
            });
        }
    }

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
