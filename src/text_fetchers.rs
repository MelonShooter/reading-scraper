pub mod text_processor;

use crabler::{ImmutableWebScraper, MutableWebScraper, Opts, Result};
use futures::future;

#[macro_export]
macro_rules! article_fetcher {
    ($pre_html_func:ident, $on_html_func:ident) => {
        async fn $on_html_func(
            &self,
            response: crabler::Response,
            element: crabler::Element,
        ) -> crabler::Result<()> {
            let article: $crate::text_fetchers::text_processor::Article =
                self.$pre_html_func(response, element).await;

            async_std::task::spawn(async {
                $crate::text_fetchers::text_processor::process_article(article);
            });

            Ok(())
        }
    };
}

/// Link fetchers should pick up as many links as they can for the article fetchers.
pub trait LinkFetcher: MutableWebScraper {
    fn get_sites(&self) -> Vec<&str>;
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

    link_fetcher
        .run(Opts::new().with_urls(link_fetcher.get_sites()))
        .await?;

    let article_fetcher = &mut text_fetcher.article_fetcher;
    let links = link_fetcher.get_links();
    let futures = links
        .iter()
        .map(|l| article_fetcher.run(Opts::new().with_urls(vec![l.as_str()])));

    future::join_all(futures).await;

    Ok(())
}

impl TextFetchers {
    pub fn new() -> TextFetchers {
        TextFetchers {
            text_fetchers: Vec::new(),
        }
    }

    /// Registers a text fetcher. Link fetchers should pick up
    /// as many links as they can for the article fetchers while
    /// the article fetchers should get all the articles it can.
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
