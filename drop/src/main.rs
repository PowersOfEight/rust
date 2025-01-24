struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data \"{}\"!", self.data);
    }
}

fn consume<T> (_: T) {
    println!("Consuming a value without doing anything with it.");
}
fn main() {
    // let c = CustomSmartPointer {
    //     data: String::from("my thingy"),
    // };

    // let d = CustomSmartPointer {
    //     data: String::from("another dinghus"),
    // };

    // println!("CustomSmartPointers created.");
    let c = CustomSmartPointer {
        data: String::from("some emoji"),
    };

    println!("CustomSmartPointer created.");
    // drop(c);
    consume(c);// c is moved here
    println!("CustomSmartPointer dropped before the end of main.");
}
