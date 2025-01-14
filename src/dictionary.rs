use std::collections::HashSet;

use rand::Rng;

use crate::char::{asciify_str, contains_utf8_only};

/// Reads text, extracts all the words and transforms it in three things:
/// A set with some misspelled words for gameplay, a set with correct words to choose secret word from, and the abecedary or all valid chars
pub fn get_words(bytes: Vec<u8>) -> (HashSet<String>, HashSet<String>, HashSet<char>) {
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
        if contains_utf8_only(s) {
            set_misspelled.insert(asciify_str(s));
        }
        set_correct.insert(s.to_owned());
    }

    set_misspelled.extend(set_correct.clone());

    (set_correct, set_misspelled, abecedary)
}

pub fn get_secret_word(words: &HashSet<String>) -> Vec<char> {
    let index: usize = rand::thread_rng().gen_range(0..words.len());
    let secret_word: Vec<char> = words
        .iter()
        .nth(index)
        .unwrap()
        .to_owned()
        .chars()
        .collect();
    secret_word
}
