pub mod text_fetchers;
pub(crate) mod util;

use async_std::sync::Mutex;
use crabler::{Element, ImmutableWebScraper, Opts, Response, Result};

use crate::text_fetchers::text_processor::difficulty::Language;
use crate::text_fetchers::text_processor::Article;
// TODO: could add return type as an argument, but thats extra boilerplate code
// cuz for articles, itll always be the same.
// or just make a proc macro that generates the entire scraper given a method - probs this
#[derive(ImmutableWebScraper)]
#[on_html("div.rich-text.single__rich-text___BlzVF > p", run)]
struct Scraper {}

impl<'a> Scraper {
    async fn pre_run(&self, response: Response, element: Mutex<Element>) -> Article {
        let element = element.lock().await;
        println!("{}", element.deep_text());

        Article::new("", response.url, "", Language::English)
    }

    article_fetcher!(pre_run, run);
}

#[async_std::main]
async fn main() -> Result<()> {
    Ok(())
}
