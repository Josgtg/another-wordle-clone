use std::collections::HashSet;

use clap::Parser;
use language::{Language, LanguagePack};

mod args;
mod char;
mod dictionary;
mod errors;
mod feedback;
mod game;
mod language;
mod prompt;
mod ascii;

fn get_language_appropriate<T>(language: &Language, english: T, spanish: T) -> T {
    match language {
        Language::English => english,
        Language::Spanish => spanish
    }
}

fn main() {
    let args = args::Args::parse();

    let language = args.get_language();

    let (lang_pack_en, lang_pack_es) = (LanguagePack::english(), LanguagePack::spanish());
    let language_pack: &LanguagePack = get_language_appropriate(&language, &lang_pack_en, &lang_pack_es);

    let (bytes_en, bytes_es) = (
        include_bytes!("../media/dictionary.txt").to_vec(),
        include_bytes!("../media/diccionario.txt").to_vec()
    );
    let (secret_en, mut words_en, abecedary_en) = dictionary::get_words(bytes_en);
    let (secret_es, mut words_es, abecedary_es) = dictionary::get_words(bytes_es);

    // The words in this set are all correct, it would not be reasonable to choose a misspelled word as the secret one
    let mut words_for_secret: &HashSet<String>;
    // Some words are misspelled in this set to improve playability, it's for spanish words with symbols
    let mut words: &mut HashSet<String>;
    // Al valid characters are in this set
    let mut abecedary: &HashSet<char>;
    
    let mut secret_word: Vec<char>;
    let mut first_time = true;
    let tries: u8 = args.get_tries();
    loop {
        words_for_secret = get_language_appropriate(&language, &secret_en, &secret_es);
        words = get_language_appropriate(&language, &mut words_en, &mut words_es);
        abecedary = get_language_appropriate(&language, &abecedary_en, &abecedary_es);

        if first_time {
            secret_word =
            if let Some(secret) = args.get_secret(abecedary) {
                words.insert(secret.iter().collect());
                secret
            } else {
                dictionary::get_secret_word(words_for_secret)
            };
            println!("{}", language_pack.welcome);
            first_time = false;
        } else {
            secret_word = dictionary::get_secret_word(words_for_secret);
        }
        
        game::start(words, &secret_word, abecedary, tries, language_pack);
        
        if !prompt::play_again(language_pack) {
            return;
        }
        println!();
    }
}
