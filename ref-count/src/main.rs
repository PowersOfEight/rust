#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(4, Rc::clone(&a));
    let c= Cons(3, Rc::clone(&a));

    // Start at C
    let mut d = &c;

    while let Cons(i, next) = d {
        println!("Current: {}", i);
        d = next;
    }
    println!("End of list: {:?}", d);
}
