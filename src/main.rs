pub mod text_fetchers;
use crabler::{async_trait, CrablerError, Element, ImmutableWebScraper, Opts, Response, Result};

use crate::text_fetchers::Article;

#[derive(ImmutableWebScraper)]
#[on_html("div.rich-text.single__rich-text___BlzVF > p", run)]
struct Scraper {}

impl Scraper {
    async fn pre_run(&self, response: Response, element: Element) -> Article {
        println!("{}", element.deep_text());

        let f = "".to_owned();

        Article::new(f.clone(), response.url, f)
    }

    article_fetcher!(pre_run, run);
}

#[async_std::main]
async fn main() -> Result<()> {
    Ok(())
}
