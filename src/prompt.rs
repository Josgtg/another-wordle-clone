use std::str::FromStr;

use crate::language::{Language, LanguagePack};
use strum::IntoEnumIterator;

use promkit::{
    preset::{
        self,
        {confirm::Confirm, listbox::Listbox, readline::Readline},
    },
    Prompt,
};

fn bool_from_str(s: &str) -> bool {
    match s {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => panic!("should not have gotten here..."),
    }
}

// Templates

fn readline(title: &str) -> Prompt<preset::readline::render::Renderer> {
    Readline::default()
        .title(title)
        .enable_history()
        .prompt()
        .expect("something went wrong when creating prompt")
}

fn listbox(title: &str) -> Prompt<preset::listbox::render::Renderer> {
    Listbox::new(Language::iter())
        .title(title)
        .listbox_lines(2)
        .prompt()
        .expect("something went wrong when creating prompt")
}

fn confirm(title: &str) -> Prompt<preset::readline::render::Renderer> {
    Confirm::new(title)
        .prompt()
        .expect("something went wrong when creating prompt")
}

// Implementations

pub fn read_input(lang: &LanguagePack) -> String {
    readline(lang.prompt_guess_title)
        .run()
        .expect("error reading user input")
        .to_lowercase()
}

pub fn change_language(lang: &LanguagePack) -> Language {
    let lang = Language::from_str(listbox(lang.prompt_change_language).run().expect("error reading user input").as_str())
        .expect("language is not valid");
    println!();
    lang
}

pub fn ask_change_language(lang: &LanguagePack) -> bool {
    let input = confirm(lang.prompt_ask_change_language).run().expect("error reading user input");
    bool_from_str(input.as_str())
}

pub fn play_again(lang: &LanguagePack) -> bool {
    let input = confirm(lang.prompt_play_again).run().expect("error reading user input");
    bool_from_str(input.as_str())
}
