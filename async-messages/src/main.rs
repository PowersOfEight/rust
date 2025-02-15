use std::time::Duration;
fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx_clone = tx.clone();
        let tx_clone_fut = async move {
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
        };

        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                println!("received \"{val}\"");
            }
        };
        
        let tx_fut = async move {
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
        };
        trpl::join3(tx_clone_fut, tx_fut, rx_fut).await;
    });
}
