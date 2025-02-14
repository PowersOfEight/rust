
fn main() {
    trpl::run( async {
        let (tx, mut rx) = trpl::channel();
        let val = String::from("hi");
        tx.send(val).unwrap();

        let recieved = rx.recv().await.unwrap();
        println!("Got: {recieved}");
    }
    );
}
