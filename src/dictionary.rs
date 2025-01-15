use std::collections::HashSet;

use rand::Rng;

use crate::ascii::{asciify_str, has_non_ascii};

pub struct Dictionary {
    /// The words in this set are all correct, it would not be reasonable to choose a misspelled word as the secret one
    words_correct: HashSet<String>,
    /// Some words are misspelled in this set to improve playability, it's for spanish words with symbols
    pub words: HashSet<String>,
    /// Al valid characters are in this set
    pub abecedary: HashSet<char>,
}

impl Dictionary {
    /// Reads text, extracts all the words and transforms it in three things:
    /// A set with some misspelled words for gameplay, a set with correct words to choose secret word from, and the abecedary or all valid chars
    pub fn new(bytes: Vec<u8>) -> Self {
        let mut set_correct: HashSet<String> = HashSet::new();
        let mut set_misspelled: HashSet<String> = HashSet::new();
        let mut abecedary: HashSet<char> = HashSet::new();

        let text: String = match std::str::from_utf8(bytes.as_ref()) {
            Ok(s) => String::from(s),
            Err(e) => panic!("there was an error while reading dictionary: {}", e),
        };

        for s in text.lines() {
            for c in s.to_lowercase().chars() {
                abecedary.insert(c);
            }
            if has_non_ascii(s) {
                set_misspelled.insert(asciify_str(s));
            }
            set_correct.insert(s.to_owned());
        }

        set_misspelled.extend(set_correct.clone());

        Self {
            words_correct: set_correct,
            words: set_misspelled,
            abecedary,
        }
    }

    pub fn get_secret_word(&self) -> Vec<char> {
        let index: usize = rand::thread_rng().gen_range(0..self.words_correct.len());
        let secret_word: Vec<char> = self
            .words_correct
            .iter()
            .nth(index)
            .unwrap()
            .to_owned()
            .chars()
            .collect();
        secret_word
    }
}
