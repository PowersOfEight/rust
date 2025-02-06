use std::sync::mpsc;
// mpsc stands for multiple producer, single consumer
use std::thread;
use std::time::Duration;
fn main() {
    // (upstream, -> downstream)
    let (producer, consumer) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi from the producer");
        let val = val
            .split_whitespace()
            .for_each(|x| {
                producer.send(x.to_string()).unwrap();
                thread::sleep(Duration::from_secs(1));
            });
        // producer.send(val).unwrap();
    });

    // let received = consumer.recv().unwrap();// blocking call
    // to use non-blocking call, use try_recv()
    for received in consumer {
        println!("Got: {received}");
    }
}
