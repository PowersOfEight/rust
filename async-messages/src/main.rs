use std::future::Future;
use std::{time::Duration, vec};
use std::pin::{pin, Pin};
use std::thread;
async fn print_letters() {
    let a = async {print!("A")};
    let b = async {print!("B")};
    let c = async {print!("C")};
    c.await;
    b.await;
    a.await;
}

fn slow(name: &str, millis: u64) {
    thread::sleep(Duration::from_millis(millis));
    println!("\"{name}\" ran for {millis}ms");
}

async fn go_speed_racer() {
    let slow = async {
        println!("\"slow\" started");
        trpl::sleep(Duration::from_millis(100)).await;
        println!("\"slow\" finished");
    };

    let fast = async {
        println!("\"fast\" started");
        trpl::sleep(Duration::from_millis(50)).await;
        println!("\"fast\" finished");
    };

    trpl::race(slow, fast).await;
}

async fn slow_simulation() {
    let one_ms = Duration::from_millis(1);
    let a = async {
        println!("\"a\" started.");
        slow("a", 30);
        trpl::sleep(one_ms).await;
        slow("a", 10);
        trpl::sleep(one_ms).await;
        slow("a", 20);
        trpl::sleep(one_ms).await;
        trpl::sleep(Duration::from_millis(50)).await;
        println!("\"a\" finished.");
    };

    let b = async {
        println!("\"b\" started.");
        slow("b", 75);
        trpl::sleep(one_ms).await;
        slow("b", 10);
        trpl::sleep(one_ms).await;
        slow("b", 15);
        trpl::sleep(one_ms).await;
        slow("b", 350);
        trpl::sleep(Duration::from_millis(50)).await;
        println!("\"b\" finished.");
    };

    trpl::race(a,b).await;
}

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx_clone = tx.clone();
        let tx_clone_fut = pin!(async move {
            let vals = vec![
                String::from("hello"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(val) = rx.recv().await {
                println!("received \"{val}\"");
            }
        });
        
        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
            for val in vals {
                tx_clone.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });
        // trpl::join!(tx_clone_fut, tx_fut, rx_fut); // works too
        // trpl::join3(tx_clone_fut, tx_fut, rx_fut).await;
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![
            tx_clone_fut,
            tx_fut, 
            rx_fut
            ];
        trpl::join_all(futures).await;
        print_letters().await;
        go_speed_racer().await;
        slow_simulation().await;
    });


}
