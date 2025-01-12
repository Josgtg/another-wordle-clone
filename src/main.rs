use std::collections::HashSet;

use language::{Language, LanguagePack};

mod char;
mod dictionary;
mod errors;
mod feedback;
mod game;
mod language;
mod prompt;

fn main() {
    let mut current_language = Language::English;
    let mut new_language: Language;

    let lang_pack_en: LanguagePack = LanguagePack::english();
    let lang_pack_es: LanguagePack = LanguagePack::spanish();
    let mut lang: &LanguagePack = &lang_pack_en;

    let bytes_en = include_bytes!("../media/dictionary.txt").to_vec();
    let bytes_es = include_bytes!("../media/diccionario.txt").to_vec();

    let (words_en, abecedary_en) = dictionary::get_words(&bytes_en);
    let (words_es, abecedary_es) = dictionary::get_words(&bytes_es);
    let mut words: &HashSet<String> = &words_en;
    let mut abecedary: &HashSet<char> = &abecedary_en;

    let mut secret_word: Vec<char>;
    let mut first_time = true;
    loop {
        new_language = prompt::ask_language(lang);
        if new_language != current_language {
            (words, abecedary, lang) = match new_language {
                language::Language::English => (&words_en, &abecedary_en, &lang_pack_en),
                language::Language::Spanish => (&words_es, &abecedary_es, &lang_pack_es),
            };
            current_language = new_language;
        }

        if first_time {
            println!("{}", lang.welcome);
            first_time = false;
        }

        secret_word = dictionary::select_secret_word(words);
        game::start(words, &secret_word, abecedary, lang);

        if !prompt::play_again(lang) {
            return;
        }
    }
}
