use crate::errors::error;
use crate::feedback::Feedback;
use crate::language::LanguagePack;
use crate::prompt::*;

use colored::Colorize;
use std::collections::HashSet;

fn validate(
    guess: &str,
    words: &HashSet<String>,
    abecedary: &HashSet<char>,
    lang: &LanguagePack,
) -> bool {
    for c in guess.chars() {
        if !abecedary.contains(&c) {
            error(lang.err_invalid_chars);
            return false;
        }
    }
    // Done like that because words with special characters are not recognized to have 5 letters with guess.len() such as árbol, for example
    if guess.chars().collect::<Vec<char>>().len() != 5 {
        error(lang.err_invalid_len);
        return false;
    }
    if !words.contains(guess) {
        error(lang.err_invalid_word);
        return false;
    }
    true
}

fn print_feedback(feedback: &Feedback) {
    println!("{}\n", feedback.to_string().as_str().bold())
}

fn print_tries_left(lang: &LanguagePack, tries: u8, max_tries: u8) {
    println!(
        "{}: {}\n",
        lang.tries_left.bold(),
        (max_tries - tries).to_string().as_str().blue().bold()
    )
}

pub fn start(
    words: &HashSet<String>,
    secret_word: &[char],
    abecedary: &HashSet<char>,
    lang: &LanguagePack,
) {
    const MAX_TRIES: u8 = 5;
    let mut guess: String;
    let mut feedback = Feedback::new(secret_word, abecedary);

    let mut tries: u8 = 0;
    'outer: loop {
        guess = read_input(lang);

        match guess.to_lowercase().as_str() {
            "q" => {
                println!();
                return;
            }
            "a" => {
                println!("\n{}\n", feedback.get_abecedary().bold());
                continue 'outer;
            }
            "w" => {
                print_feedback(&feedback);
                continue 'outer;
            }
            "t" => {
                print_tries_left(lang, tries, MAX_TRIES);
                continue 'outer;
            }
            "h" => {
                println!("{}\n", lang.commands);
                continue 'outer;
            }
            _ => ()
        };

        if !validate(&guess, words, abecedary, lang) {
            continue;
        }

        feedback.compare(guess);
        print_feedback(&feedback);

        if feedback.win {
            println!("{}\n", lang.win.bold());
            return;
        } else if tries == MAX_TRIES - 1 {
            println!(
                "{}: \"{}\"\n",
                lang.loss.bold(),
                feedback.get_secret().to_uppercase().bold()
            );
            return;
        }

        tries += 1;
        print_tries_left(lang, tries, MAX_TRIES);
    }
}
