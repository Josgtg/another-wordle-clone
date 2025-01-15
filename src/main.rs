use clap::Parser;

mod args;
mod char;
mod dictionary;
mod errors;
mod feedback;
mod game;
mod language;
mod prompt;
mod ascii;

use language::{Language, LanguagePack};
use dictionary::Dictionary;

fn get_language_appropriate<T>(language: &Language, english: T, spanish: T) -> T {
    match language {
        Language::English => english,
        Language::Spanish => spanish
    }
}

fn main() {
    let args = args::Args::parse();

    let mut language = args.get_language();

    let mut language_pack: LanguagePack = get_language_appropriate(
        &language,
        LanguagePack::english(),
        LanguagePack::spanish()
    );

    let mut dictionary: Dictionary = get_language_appropriate(
        &language,
        Dictionary::new(include_bytes!("../media/dictionary.txt").to_vec()),
        Dictionary::new(include_bytes!("../media/diccionario.txt").to_vec())
    );
    
    let mut first_time = true;
    loop {
        if first_time {
            println!("{}", language_pack.welcome);
            first_time = false;
            if let Some(secret) = args.get_secret(&dictionary.abecedary) {
                dictionary.words.insert(secret.iter().collect());
                game::start(&dictionary.words, &secret, &dictionary.abecedary, args.get_tries(), &language_pack);
                continue;
            }
        }
        
        game::start(&dictionary.words, &dictionary.get_secret_word(), &dictionary.abecedary, args.get_tries(), &language_pack);
        
        if !prompt::play_again(&language_pack) {
            return;
        }
        if prompt::ask_change_language(&language_pack) {
            language = prompt::change_language(&language_pack);
            language_pack = get_language_appropriate(
                &language,
                LanguagePack::english(),
                LanguagePack::spanish()
            );
            dictionary = get_language_appropriate(
                &language,
                Dictionary::new(include_bytes!("../media/dictionary.txt").to_vec()),
                Dictionary::new(include_bytes!("../media/diccionario.txt").to_vec())
            );
            first_time = true;
        }
        println!();
    }
}
