enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let mut n = 1;
    let b = Box::new(&mut n);
    **b += 1;
    println!("{}", n);
}
