use std::collections::HashSet;

const STOP_WORDS: &str = include_str!("analyzer/stop_words.txt");

pub struct Analyzer {
    /// known stop words.
    stop_words: HashSet<String>,
}

impl Analyzer {
    /// Construct a new analyzer.
    pub fn new() -> Self {
        let mut stop_words = HashSet::new();

        for word in STOP_WORDS.split('\n') {
            let word = word.trim();

            if !word.is_empty() {
                stop_words.insert(word.to_owned());
            }
        }

        Self { stop_words }
    }

    /// Filter a word with the analyzer.
    pub(crate) fn filter<'a>(&'a self, word: &'a str) -> impl Iterator<Item = Box<str>> + 'a {
        let it = word.split(split_fn);

        return it.filter_map(move |word| {
            let word = word.to_lowercase();

            if self.stop_words.contains(&word) {
                None
            } else {
                Some(word.into())
            }
        });

        fn split_fn(c: char) -> bool {
            if c.is_whitespace() {
                return true;
            }

            matches!(c, '/' | '-')
        }
    }
}
