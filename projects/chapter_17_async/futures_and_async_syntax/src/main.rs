use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // the main function cannot be async
    // all async functions need a runtime to set them up
    // so we use trpl::block_on, which sets up a runtime evaluates the async code
    trpl::block_on( async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) = 
        match trpl::select(title_fut_1, title_fut_2).await {
            // an enum type when there isn't a heirarchy of which is returned first
            // is has two variants: Left and Right, of any type each
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("url returned first!");
        match page_title(url).await {
            Some(title) => println!("The page title for {url} is {title}"),
            None => println!("{url} doesn't have a title")
        }
    })
}
