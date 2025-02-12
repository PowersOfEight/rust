use std::future::Future;
use trpl::Html;


// Equivalent function shown below
async fn page_title(url: &str) -> Option<String> {
    // let response = trpl::get(url).await;
    // let response_text = response.text().await;
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(
            |title_element| title_element.inner_html()
        )
}


// Equivalent async function shown below
fn page_title_equivalent(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        Html::parse(
            &trpl::get(url)
                .await
                .text()
                .await
        ).select_first("title")
        .map(

            |title_element| title_element.inner_html()
        )
    }
}

fn main() {
}
