use std::collections::HashSet;

use rand::Rng;

use crate::ascii::{asciify_str, has_non_ascii};

pub struct Dictionary {
    /// The words in this set are all correct, it would not be reasonable to choose a misspelled word as the secret one
    secret_words: HashSet<String>,
    /// Some words are misspelled in this set to improve playability, it's for spanish words with symbols
    pub dictionary: HashSet<String>,
    /// Al valid characters are in this set
    pub abecedary: HashSet<char>,
}

impl Dictionary {
    /// Reads text, extracts all the words and transforms it in three things:
    /// A set with some misspelled words for gameplay, a set to choose secret word from, and the abecedary containing all valid chars
    pub fn new(dictionary_bytes: &[u8], secret_word_bytes: &[u8]) -> Self {
        let mut set_secret: HashSet<String> = HashSet::new();
        let mut set_dictionary: HashSet<String> = HashSet::new();
        let mut abecedary: HashSet<char> = HashSet::new();

        let text_for_dictionary: String = match std::str::from_utf8(dictionary_bytes) {
            Ok(s) => String::from(s),
            Err(e) => panic!("there was an error while reading dictionary: {}", e),
        };

        for s in text_for_dictionary.lines() {
            for c in s.to_lowercase().chars() {
                abecedary.insert(c);
            }
            if has_non_ascii(s) {
                set_dictionary.insert(asciify_str(s));
            }
            set_dictionary.insert(s.to_owned());
        }

        let text_for_secret: String = match std::str::from_utf8(secret_word_bytes) {
            Ok(s) => String::from(s),
            Err(e) => panic!("there was an error while reading dictionary: {}", e),
        };

        for s in text_for_secret.lines() {
            for c in s.to_lowercase().chars() {
                abecedary.insert(c);
            }
            set_secret.insert(s.to_owned());
        }

        Self {
            secret_words: set_secret,
            dictionary: set_dictionary,
            abecedary,
        }
    }

    pub fn get_secret_word(&self) -> Vec<char> {
        let index: usize = rand::thread_rng().gen_range(0..self.secret_words.len());
        let secret_word: Vec<char> = self
            .secret_words
            .iter()
            .nth(index)
            .unwrap()
            .to_owned()
            .chars()
            .collect();
        secret_word
    }
}
