use std::future::Future;
use trpl::{Html, Either};

// Equivalent function shown below
async fn page_title(url: &str) -> (&str, Option<String>) {
    // let response = trpl::get(url).await;
    // let response_text = response.text().await;
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

// Equivalent async function shown below
fn page_title_equivalent(url: &str) -> impl Future<Output = (&str, Option<String>)> + '_ {
    async move {
       let title = Html::parse(&trpl::get(url).await.text().await)
            .select_first("title")
            .map(|title_element| title_element.inner_html());
        (url, title)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title_equivalent(&args[2]);

        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("\"{url}\" returned first");
        match maybe_title {
            Some(title ) => println!("The page title is \"{title}\""),
            None => println!("The page has no title"),
        }
    })
}

