use std::fmt::Debug;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::env::args;

fn main() {

    let mut n = 1;
    let t = thread::spawn(move || {
        n = n + 1;
        thread::spawn(move || {
            n = n + 1;
        }); 
    });
    n = n + 1;
    t.join().unwrap();
    println!("{n}");
    // if let Some(name) = args().nth(0) {
    //     println!("Threading \"{name}\"");
    //     let mut join_handles= vec![];
    //     for letter in name.chars() {

    //         let handle = thread::spawn(move || {
    //             thread::sleep(Duration::from_millis(1));
    //             println!("{letter}");
    //         });
    //         join_handles.push(handle);
    //     }

    //     for handle in join_handles {
    //         match handle.join() {
    //             Ok(_) => println!("Thread finished"),
    //             Err(_) => println!("Thread panicked"),
    //         }
    //     }
    // }

    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap();
    // for i in 1..5 {
    //     println!("hi number {i} from the main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }

}
