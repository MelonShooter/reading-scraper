mod text_processor;

use crabler::{ImmutableWebScraper, MutableWebScraper, Opts, Result};
use futures::future;

pub trait LinkFetcher: MutableWebScraper {
    fn get_site(&self) -> String;
    fn get_links(&self) -> Vec<String>;
}

struct TextFetcher {
    link_fetcher: Box<dyn LinkFetcher>,
    article_fetcher: Box<dyn ImmutableWebScraper>,
}

pub struct TextFetchers {
    text_fetchers: Vec<TextFetcher>,
}

async fn execute_text_fetcher(text_fetcher: &mut TextFetcher) -> Result<()> {
    let link_fetcher = &mut text_fetcher.link_fetcher;
    let site = link_fetcher.get_site();

    link_fetcher
        .run(Opts::new().with_urls(vec![site.as_str()]))
        .await?;

    let article_fetcher = &mut text_fetcher.article_fetcher;
    let links = link_fetcher.get_links();
    let futures = links
        .iter()
        .map(|l| article_fetcher.run(Opts::new().with_urls(vec![l.as_str()])));

    future::join_all(futures).await;
    // Make a macro that implements the run method properly for the ImmutableWebScraper
    // the run method implemented should use the text gotten from a pre_run method and pass it through the text processor for the article within a new task

    Ok(())
}

impl TextFetchers {
    pub fn new() -> TextFetchers {
        TextFetchers {
            text_fetchers: Vec::new(),
        }
    }

    pub fn register(
        &mut self,
        link_fetcher: Box<dyn LinkFetcher>,
        article_fetcher: Box<dyn ImmutableWebScraper>,
    ) -> &mut TextFetchers {
        let text_fetcher = TextFetcher {
            link_fetcher,
            article_fetcher,
        };

        self.text_fetchers.push(text_fetcher);

        self
    }

    pub async fn start(&mut self) {
        let executing_text_fetchers = self
            .text_fetchers
            .iter_mut()
            .map(|text_fetcher| execute_text_fetcher(text_fetcher));

        future::join_all(executing_text_fetchers).await;

        // TODO: Verify that this actually executes everything concurrently.
    }
}
