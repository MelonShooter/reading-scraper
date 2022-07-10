pub mod difficulty;

use async_std::fs::{File, OpenOptions};
use lazy_static::lazy_static;
use regex::Regex;
use std::io::Result;
use std::ops::Range;
use std::sync::Arc;

use self::difficulty::Language;

const NOT_SENTENCE_REGEX_STRING: &str = r"(?:Dr|Mr?s?|\.? ?[A-Z]{1,2})$";

struct ArticleChunkTracker {
    len: usize,
    word_count: usize,
}

impl ArticleChunkTracker {
    fn new() -> Self {
        Self {
            len: 0,
            word_count: 0,
        }
    }

    fn add_sentence(&mut self, sentence: &str) {
        self.len += sentence.len();
        self.word_count += sentence.split(" ").count();
    }

    fn reset(&mut self) {
        self.len = 0;
        self.word_count = 0;
    }
}

pub struct Article {
    title: Arc<str>,
    link: Arc<str>,
    body: Arc<str>,
    range: Range<usize>,
    language: Language,
}

impl Article {
    pub fn new(
        title: impl ToString,
        link: impl ToString,
        body: impl ToString,
        language: Language,
    ) -> Article {
        Self::new_internal(
            title.to_string(),
            link.to_string(),
            body.to_string(),
            language,
        )
    }

    fn new_internal(title: String, link: String, body: String, language: Language) -> Article {
        lazy_static! {
            static ref WHITESPACE_MATCH: Regex = Regex::new(r"\s+").unwrap();
        };

        let body = WHITESPACE_MATCH
            .replace_all(body.as_str(), " ")
            .into_owned();
        let body_len = body.len();

        Article {
            title: Arc::from(title),
            link: Arc::from(link),
            body: Arc::from(body),
            range: 0..body_len,
            language,
        }
    }

    fn get_body(&self) -> &str {
        &self.body[self.range.clone()]
    }

    pub fn to_chunks(self, approximate_word_count: usize) -> Vec<Article> {
        let mut chunks = Vec::new();

        let sentence_iterator = self.body.split(". ");

        let mut chunk_tracker = ArticleChunkTracker::new();

        lazy_static! {
            static ref NOT_SENTENCE_REGEX: Regex = Regex::new(NOT_SENTENCE_REGEX_STRING).unwrap();
        }

        let mut current_idx = 0usize;

        // Use chunk length to determine how much to push
        for sentence in sentence_iterator {
            chunk_tracker.add_sentence(sentence);

            // This means that this is a sentence boundary and the approximate word count has been reached.
            if !NOT_SENTENCE_REGEX.is_match(sentence)
                && chunk_tracker.word_count >= approximate_word_count
            {
                let next_idx = current_idx + chunk_tracker.len;

                chunks.push(Article {
                    title: self.title.clone(),
                    link: self.link.clone(),
                    body: self.body.clone(),
                    range: current_idx..next_idx,
                    language: self.language,
                });

                chunk_tracker.reset();
                current_idx = next_idx;
            }
        }

        chunks
    }
}

async fn create_article_file(file_name: &str) -> Result<File> {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .await
}
