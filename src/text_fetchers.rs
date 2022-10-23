pub mod text_processor;

use std::iter;
use tokio::sync::{self, Receiver, Sender};
use tokio::task;

use self::text_processor::Article;

pub const CHUNK_WORD_COUNT: usize = 100;

#[macro_export]
macro_rules! article_fetcher {
    ($pre_html_func:ident, $on_html_func:ident) => {
        async fn $on_html_func(
            &self,
            response: crabler::Response,
            element: async_std::sync::Mutex<crabler::Element>,
        ) -> Vec<$crate::text_fetchers::text_processor::Article> {
            let article: $crate::text_fetchers::text_processor::Article =
                self.$pre_html_func(response, element).await;

            article.to_chunks($crate::text_fetchers::CHUNK_WORD_COUNT)
        }
    };
}

type ArticleReturnType = Vec<Article>;
type LinkReturnType = Vec<String>;

/// TODO: Make ReturnType more flexible so it can be anything
/// might be able to remove the mutable web scrapers
/// clean up crabler, just using immutable ones, removing unused functionality, and documenting existing necessary functionality

/// Link fetchers should pick up as many links as they can for the article fetchers.

struct TextFetcher {
    sources: Vec<String>,
    link_fetcher: Box<dyn ImmutableWebScraper<ReturnType = LinkReturnType> + Send + Sync>,
    article_fetcher: Box<dyn ImmutableWebScraper<ReturnType = ArticleReturnType> + Send + Sync>,
}

fn execute_text_fetcher(sender: &Sender<Article>, text_fetcher: TextFetcher) {
    for source in text_fetcher.sources {
        task::spawn(async {
            let opts = Opts::new().with_urls(iter::once(source));
            let scraper = text_fetcher.link_fetcher.run(opts).await;
        });
    }
}

pub struct TextFetchers(Vec<TextFetcher>);

impl TextFetchers {
    pub fn new() -> TextFetchers {
        TextFetchers(Vec::new())
    }

    /// Registers a text fetcher. Link fetchers should pick up
    /// as many links as they can for the article fetchers while
    /// the article fetchers should get all the articles it can.
    pub fn register<T: 'static, U: 'static>(
        &mut self,
        sources: Vec<String>,
        link_fetcher: T,
        article_fetcher: U,
    ) -> &mut TextFetchers
    where
        T: ImmutableWebScraper<ReturnType = LinkReturnType> + Send + Sync,
        U: ImmutableWebScraper<ReturnType = ArticleReturnType> + Send + Sync,
    {
        let text_fetcher = TextFetcher {
            sources,
            link_fetcher: Box::new(link_fetcher),
            article_fetcher: Box::new(article_fetcher),
        };

        self.0.push(text_fetcher);

        self
    }

    pub fn run(self) -> Receiver<Article> {
        let (sender, receiver) = channel::unbounded();

        for text_fetcher in self.0 {
            execute_text_fetcher(&sender, text_fetcher);
        }

        receiver
    }
}
