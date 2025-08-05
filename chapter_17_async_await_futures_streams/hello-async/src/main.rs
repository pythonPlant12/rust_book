use std::env::args;

use trpl::Html;

fn main() {
    // We cannot call here:
    // async function() {}
    // function().await .... because the async needs runtime and is not implemented in the main
    // function in Rust by default.
    // We need to use for it async runtime like tokio.
    // await in rust is written at the end. This allows to have multiple awaits like:
    // function().await.function().await;
    // When the function is async, it compiles in into a unique, anonymous data type that
    // implements the `Future` trait. (Future is something like Promise in JS).
    //
    // Most programming programming languages support async bundle a runtime, but Rust does not.
    //
    // here we'll use run function from trpl (Rust for study). which initialize the async runtime
    // with Tokio.
    let args: Vec<String> = args().collect();

    fn page_title(url: &str) -> impl Future<Output = Option<String>> {
        async move {
            let text = trpl::get(url).await.text().await;
            Html::parse(&text)
            .select_first("title")
                .map(|title| title.inner_html())
        }
    }
    
    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title")
        }
    });
}
