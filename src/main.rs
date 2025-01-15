use std::io::Write;
use clap::Parser;

mod args;
mod ascii;
mod char;
mod dictionary;
mod feedback;
mod game;
mod language;
mod prompt;

use dictionary::Dictionary;
use language::{Language, LanguagePack};

fn get_language_appropriate<T>(language: &Language, english: T, spanish: T) -> T {
    match language {
        Language::English => english,
        Language::Spanish => spanish,
    }
}

fn change_language(language: &Language) -> (LanguagePack, Dictionary) {
    (
        get_language_appropriate(language, LanguagePack::english(), LanguagePack::spanish()),
        get_language_appropriate(
            language,
            Dictionary::new(include_bytes!("../media/dictionary.txt").to_vec()),
            Dictionary::new(include_bytes!("../media/diccionario.txt").to_vec()),
        ),
    )
}

fn first_time_secret(args_secret: Option<Vec<char>>, dictionary: &mut Dictionary) -> Vec<char> {
    if let Some(secret) = args_secret {
        dictionary.words.insert(secret.iter().collect());
        secret
    } else {
        dictionary.get_secret_word()
    }
}

fn print_welcome(welcome: &str) {
    print!("{}", welcome);
    std::io::stdout().flush().expect("Could not flush stdout");
    std::io::stdin().read_line(&mut String::new()).expect("Could not read line");
    println!();
}


fn main() {
    let args = args::Args::parse();

    let mut language = args.get_language();
    let (mut language_pack, mut dictionary) = change_language(&language);

    let mut secret_word: Vec<char>;
    let mut first_time = true;
    loop {
        secret_word = if first_time {
            print_welcome(language_pack.welcome);
            first_time = false;
            first_time_secret(args.get_secret(&dictionary.abecedary), &mut dictionary)
        } else {
            dictionary.get_secret_word()
        };

        game::start(
            &dictionary.words,
            &secret_word,
            &dictionary.abecedary,
            args.get_tries(),
            &language_pack,
        );

        if !prompt::play_again(&language_pack) {
            return;
        }
        if prompt::ask_change_language(&language_pack) {
            language = prompt::change_language(&language_pack);
            (language_pack, dictionary) = change_language(&language);
            first_time = true;
        }
        println!();
    }
}
