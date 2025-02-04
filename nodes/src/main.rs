use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let r1 = Rc::new(0);
    let r4 = {
        let r2 = Rc::clone(&r1);
        Rc::downgrade(&r2)
    };

    let r5 = Rc::clone(&r1);
    let r6 = r4.upgrade();
    println!("{} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));

//     let leaf = Rc::new( Node {
//         value: 3,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![]),
//     });

//     println!( //<-- this is the stopping point, listing 15-29
//         "leaf strong = {}, weak = {}",
//         Rc::strong_count(&leaf),
//         Rc::weak_count(&leaf),
//     );

//     // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
// {

//     let branch = Rc::new(Node {
//         value: 5,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![Rc::clone(&leaf)]),
//     });
    
//     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

//     println!("branch strong = {}, weak = {}",
//         Rc::strong_count(&branch),
//         Rc::weak_count(&branch), 
//     );

//     println!("leaf strong = {}, weak = {}",
//         Rc::strong_count(&leaf),
//         Rc::weak_count(&leaf),
//     );
// }


//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
//     println!("branch strong = {}, weak = {}",
//         Rc::strong_count(&leaf),
//         Rc::weak_count(&leaf),
//     );
}
