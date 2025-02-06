use std::sync::mpsc;
// mpsc stands for multiple producer, single consumer
use std::thread;
fn main() {
    // (upstream, -> downstream)
    let (producer, consumer) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi from the producer");
        producer.send(val).unwrap();
        println!("Value sent: {val}");
    });

    let received = consumer.recv().unwrap();// blocking call
    // to use non-blocking call, use try_recv()
    println!("Got: {received}");
}
