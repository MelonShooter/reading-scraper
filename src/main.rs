pub mod text_fetchers;
use crabler::{async_trait, CrablerError, Element, ImmutableWebScraper, Opts, Response, Result};

#[derive(ImmutableWebScraper)]
#[on_html("div.rich-text.single__rich-text___BlzVF > p", run)]
struct Scraper {}

impl Scraper {
    async fn pre_run(&self, _: Response, element: Element) -> String {
        println!("{}", element.deep_text());

        "".to_owned()
    }

    article_fetcher!(pre_run, run);
}

#[async_std::main]
async fn main() -> Result<()> {
    Ok(())
}
