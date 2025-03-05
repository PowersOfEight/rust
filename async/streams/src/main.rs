extern crate trpl;

use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

async fn stream_example_one() {
    let values = (1..11).into_iter().collect::<Vec<i32>>();
    let iter = values.iter().map(|x| x * 2);
    let mut stream = trpl::stream_from_iter(iter);

    while let Some(value) = stream.next().await {
        println!("The value was: {value}");
    }
}


async fn stream_example_two() {
    let stream = trpl::stream_from_iter(
        (1..101).into_iter().map(|x| x * 2)
    );

    let mut filtered = 
        stream.filter(
            |val| val % 3 == 0 || val % 5 == 0
        );

    while let Some(x) = filtered.next().await {
        println!("The value was: {x}");
    }
}

async fn stream_example_three() {
    let mut stream = get_messages();

    while let Some(message) = stream.next().await {
        println!("{message}");
    }
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        for (index, message) in (0..10)
            .into_iter().map(|x| ('a' as u8 + x) as char)
            .enumerate() {
                let time_to_sleep = if index & 1 == 0 { 100 } else { 300 };
                trpl::sleep(Duration::from_millis(time_to_sleep)).await;
                if let Err(send_error) = tx.send(format!("Message: {message}")) {
                    eprintln!("Cannot send message '{message}': {send_error}");
                    break;
                }
            }

    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = i32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task( async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(send_err) = tx.send(count){
                eprintln!("Cannot send interval {count}: {send_err}");
                break;
            }
        }
    });
    ReceiverStream::new(rx)
}

async fn stream_example_four() -> () {
    let mut messages=
        pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => println!("Problem: {reason:?}"),
            }
        }
}



fn main() {
    trpl::run(async {
        stream_example_one().await;
        stream_example_two().await;
        stream_example_three().await;
        stream_example_four().await;
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|x| format!("Interval: #{x}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);
        while let Some(message) = stream.next().await {
            match message {
                Ok(value) => println!("{value}"),
                Err(reason) => println!("Problem: {reason:?}"),
            }
        }
    });
}
