mod text_fetchers;
use crabler::{async_trait, CrablerError, Element, ImmutableWebScraper, Opts, Response, Result};
use std::compile_error;

#[derive(ImmutableWebScraper)]
#[on_html("div.rich-text.single__rich-text___BlzVF > p", print_handler)]
struct Scraper {}

impl Scraper {
    async fn print_handler(&self, _: Response, element: Element) -> Result<()> {
        println!("{}", element.deep_text());

        Ok(())
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    Ok(())
}
