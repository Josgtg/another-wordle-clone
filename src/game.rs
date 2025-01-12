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

fn print_feedback(feedback: &Feedback, lang: &LanguagePack) {
    let word = feedback.to_string().as_str().bold();
    if word.trim().is_empty() {
        println!("{}\n", lang.no_word_guessed.bold());
        return;
    }
    println!("{}\n", word);
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
    const MAX_TRIES: u8 = 6;
    let mut guess: String;
    let mut feedback = Feedback::new(secret_word, abecedary);

    let mut tries: u8 = 0;
    'outer: loop {
        guess = read_input(lang);
        if guess.trim().is_empty() {
            continue;
        }

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
                // Shows previous word
                print_feedback(&feedback, lang);
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
            "l" => {
                // Show history
                let history = feedback.get_history();
                if history.is_empty() {
                    println!("{}\n", lang.no_word_guessed.bold());
                } else {
                    println!();
                    history.iter().for_each(|s| println!("{}\n", s));
                }
                continue 'outer;
            }
            _ => (),
        };

        if !validate(&guess, words, abecedary, lang) {
            continue;
        }

        feedback.compare(guess);

        if feedback.win {
            println!(
                "\n{}\n",
                feedback
                    .get_secret()
                    .to_uppercase()
                    .chars()
                    .fold(String::new(), |mut s, c| {
                        s.push(' ');
                        s.push(c);
                        s
                    })
                    .as_str()
                    .green()
                    .bold()
            );
            println!("{}\n", lang.win.bold());
            return;
        } else if tries == MAX_TRIES - 1 {
            print_feedback(&feedback, lang);
            println!(
                "{}: \"{}\"\n",
                lang.loss.bold(),
                feedback.get_secret().to_uppercase().bold()
            );
            return;
        }

        print_feedback(&feedback, lang);

        tries += 1;
        print_tries_left(lang, tries, MAX_TRIES);
    }
}
