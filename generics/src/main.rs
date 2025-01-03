use std::cmp::PartialOrd;
pub mod lifetimes;

use crate::lifetimes::generic_lifetimes::main as lifetime_main;


fn main() {
    let list = vec![34, 50, 25, 100, 65];
    let result = largest(&list);
    println!("The largest number is {result}");
    let list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&list);
    println!("The largest char is {result}");

    lifetime_main();

    
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];

    for n in list {
        if n > biggest {
            biggest = n;
        }
    }

    biggest
}
