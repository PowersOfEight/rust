#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a: {}", Rc::strong_count(&a));
    let b = Cons(4, Rc::clone(&a));
    println!("Count after creating b: {}", Rc::strong_count(&a));
    {
        let c = Cons(3, Rc::clone(&a));
        println!("Count after creating c: {}", Rc::strong_count(&a));
        dbg!(c);
    }
    // let c= Cons(3, Rc::clone(&a));
    println!("Count after c goes out of scope: {}", Rc::strong_count(&a));
    // Start at b
    let mut d = &b;

    while let Cons(i, next) = d {
        println!("Current: {}", i);
        d = next;
    }
    println!("End of list: {:?}", d);
    q1();
}

struct Example;

impl Drop for Example {
    fn drop(&mut self) {
        println!("drop");
    }
}
fn q1() -> () {
    let x = Rc::new(Example);
    let y = Rc::clone(&x);
    println!("A");
    drop(x);
    println!("B");
    drop(y);
    println!("C")
}
