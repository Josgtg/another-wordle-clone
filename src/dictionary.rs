use std::collections::HashSet;

use rand::Rng;

pub fn get_words(bytes: &Vec<u8>) -> (HashSet<String>, HashSet<char>) {
    let mut set: HashSet<String> = HashSet::new();
    let mut abecedary: HashSet<char> = HashSet::new();
    let text: String = match std::str::from_utf8(bytes.as_ref()) {
        Ok(s) => String::from(s),
        Err(e) => panic!("there was an error while reading dictionary: {}", e),
    };

    for s in text.lines() {
        for c in s.to_lowercase().chars() {
            abecedary.insert(c);
        }
        set.insert(s.to_owned());
    }
    (set, abecedary)
}

pub fn select_secret_word(words: &HashSet<String>) -> Vec<char> {
    let index: usize = rand::thread_rng().gen_range(0..words.len());
    let word: String = words.iter().nth(index).unwrap().to_owned();

    word.chars().collect()
}
