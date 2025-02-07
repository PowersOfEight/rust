use std::sync::{Mutex, Arc};
use std::thread;
// use std::{sync::Arc, thread};
fn main() {
    // let s = String::from("Hello world");
    // let a = Arc::new(&s);
    // let a2 = Arc::clone(&a);
    // let t = thread::spawn(move || {
    //     a2.len()
    // });
    // let len = t.join().unwrap();
    // println!("{} {}")
    let counter= Arc::new(Mutex::new(0));
    let mut handles= vec![];

    for _ in 0..10 {
        let counter= Arc::clone(&counter);
        let handle = thread::spawn(
            move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        );
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    

}
