pub mod text_fetchers;
pub(crate) mod util;

use crabler::{async_trait, CrablerError, Element, ImmutableWebScraper, Opts, Response, Result};

use crate::text_fetchers::text_processor::Article;

#[derive(ImmutableWebScraper)]
#[on_html("div.rich-text.single__rich-text___BlzVF > p", run)]
struct Scraper {}

impl<'a> Scraper {
    async fn pre_run(&self, response: Response, element: Element) -> Article<'a> {
        println!("{}", element.deep_text());

        Article::new(String::new(), response.url, String::new())
    }

    article_fetcher!(pre_run, run);
}

#[async_std::main]
async fn main() -> Result<()> {
    Ok(())
}
