struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data \"{}\"!", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my thingy"),
    };

    let d = CustomSmartPointer {
        data: String::from("another dinghus"),
    };

    println!("CustomSmartPointers created.");
}
