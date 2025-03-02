extern crate trpl;

use trpl::StreamExt;

async fn stream_example_one() {
    let values = (1..11).into_iter().collect::<Vec<i32>>();
    let iter = values.iter().map(|x| x * 2);
    let mut stream = trpl::stream_from_iter(iter);

    while let Some(value) = stream.next().await {
        println!("The value was: {value}");
    }
}
fn main() {
    trpl::run(async {
        stream_example_one().await;
    });
}
