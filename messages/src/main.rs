use std::sync::mpsc;
// mpsc stands for multiple producer, single consumer
use std::thread;
use std::time::Duration;
fn main() {
    // (upstream, -> downstream)
    let (producer, consumer) = mpsc::channel();

    let producer_clone = producer.clone();
    thread::spawn(move || {
        String::from("hi from the producer")
            .split_whitespace()
            .for_each(|x| {
                producer_clone.send(x.to_string()).unwrap();
                thread::sleep(Duration::from_secs(1));
            });
        // producer.send(val).unwrap();
    });

    thread::spawn(move || {
        String::from("More messages from another producer")
            .split_whitespace()
            .for_each(|x| {
                producer.send(x.to_string()).unwrap();
                thread::sleep(Duration::from_secs(1));
            });
    });

    // let received = consumer.recv().unwrap();// blocking call
    // to use non-blocking call, use try_recv()
    for received in consumer {
        println!("Got: {received}");
    }
}
