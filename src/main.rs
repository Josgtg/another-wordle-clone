use std::collections::HashSet;

use language::{Language, LanguagePack};

mod char;
mod dictionary;
mod errors;
mod feedback;
mod game;
mod language;
mod prompt;
mod ascii;

fn main() {
    let mut current_language = Language::English;
    let mut new_language: Language;

    let lang_pack_en: LanguagePack = LanguagePack::english();
    let lang_pack_es: LanguagePack = LanguagePack::spanish();
    let mut lang: &LanguagePack = &lang_pack_en;

    let bytes_en = include_bytes!("../media/dictionary.txt").to_vec();
    let bytes_es = include_bytes!("../media/diccionario.txt").to_vec();
    let (secret_en, words_en, abecedary_en) = dictionary::get_words(bytes_en);
    let (secret_es, words_es, abecedary_es) = dictionary::get_words(bytes_es);
    // The words in this set are all correct, it would not be reasonable to choose a misspelled word as the secret one
    let mut words_for_secret: &HashSet<String> = &secret_en;
    // Some words are misspelled in this set to improve playability, it's for spanish words with symbols
    let mut words: &HashSet<String> = &words_en;
    // Al valid characters are in this set
    let mut abecedary: &HashSet<char> = &abecedary_en;

    let mut secret_word: Vec<char>;
    let mut first_time = true;
    loop {
        new_language = prompt::ask_language(lang);
        if new_language != current_language {
            (words_for_secret, words, abecedary, lang) = match new_language {
                language::Language::English => {
                    (&secret_en, &words_en, &abecedary_en, &lang_pack_en)
                }
                language::Language::Spanish => {
                    (&secret_es, &words_es, &abecedary_es, &lang_pack_es)
                }
            };
            current_language = new_language;
        }

        if first_time {
            println!("{}", lang.welcome);
            first_time = false;
        }

        secret_word = dictionary::get_secret_word(words_for_secret);
        game::start(words, &secret_word, abecedary, lang);

        if !prompt::play_again(lang) {
            return;
        }
    }
}
