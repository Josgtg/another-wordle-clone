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

fn readline(lang: &LanguagePack) -> Prompt<preset::readline::render::Renderer> {
    Readline::default()
        .title(lang.prompt_guess_title)
        .enable_history()
        .prompt()
        .expect("something went wrong when creating prompt")
}

pub fn read_input(lang: &LanguagePack) -> String {
    readline(lang)
        .run()
        .expect("error reading user input")
        .to_lowercase()
}

fn listbox(lang: &LanguagePack) -> Prompt<preset::listbox::render::Renderer> {
    Listbox::new(Language::iter())
        .title(lang.prompt_language_title)
        .listbox_lines(2)
        .prompt()
        .expect("something went wrong when creating prompt")
}

pub fn ask_language(lang: &LanguagePack) -> Language {
    let lang = Language::from_str(listbox(lang).run().expect("error reading user input").as_str())
        .expect("language is not valid");
    println!();
    lang
}

fn confirm(lang: &LanguagePack) -> Prompt<preset::readline::render::Renderer> {
    Confirm::new(lang.prompt_play_again)
        .prompt()
        .expect("something went wrong when creating prompt")
}

pub fn play_again(lang: &LanguagePack) -> bool {
    let input = confirm(lang).run().expect("error reading user input");
    match input.as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => panic!("should not have gotten here..."),
    }
}
