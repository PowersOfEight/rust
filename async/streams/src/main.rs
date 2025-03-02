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
fn main() {
    trpl::run(async {
        stream_example_one().await;
        stream_example_two().await;
    });
}
