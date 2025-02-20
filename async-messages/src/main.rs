use std::future::Future;
use std::{time::Duration, vec};
use std::pin::{pin, Pin};
async fn print_letters() {
    let a = async {print!("A")};
    let b = async {print!("B")};
    let c = async {print!("C")};
    c.await;
    b.await;
    a.await;
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
    });


}
