use std::borrow::Cow;
use std::rc::Rc;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Article<'a> {
    title: String,
    link: String,
    body: Cow<'a, str>,
}

impl<'a> Article<'a> {
    pub fn new(title: String, link: String, body: String) -> Article<'a> {
        lazy_static! {
            static ref WHITESPACE_MATCH: Regex = Regex::new(r"\s+").unwrap();
        };

        let body = Cow::from(
            WHITESPACE_MATCH
                .replace_all(body.as_str(), " ")
                .into_owned(),
        );

        Article { title, link, body }
    }

    pub(crate) fn to_chunks(
        self,
        approximate_word_count: usize,
        minimum_word_count: usize,
    ) -> Vec<Article<'a>> {
        let chunks = Vec::new();
        let body_length = self.body.len();

        let sentence_iterator = self.body.split('.').map(|sentence| {
            if sentence.is_char_boundary(1) && &sentence[0..1] == " " {
                &sentence[1..]
            } else {
                sentence
            }
        });

        for sentence in sentence_iterator {
            // check abbreviations at the end of sentences, if its at the end, merge it with the next slice by getting the index of the first string and
            // once the end of the sentence is determined, then, do the check for word count
            // if word count is at approximate, add it to article vector - figure out how to make this work without copying, maybe a Cow<String>
        }

        // then filter out last element if it ends up being less than the min

        chunks
    }
}

pub fn process_article(article: Article) {
    // to process an article
    // 1. split the article up into chunks
    // 2. determine difficulty
}
