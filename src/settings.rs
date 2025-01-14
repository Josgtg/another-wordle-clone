use json::parse;

use colored::Colorize;
use crate::{args::Args, language::Language};

pub struct Settings {
    pub language: Language,
    pub secret: Vec<char>,
    pub tries: u32
}

impl<A: Args> From<A> for Settings {
    fn from(value: A) -> Self {
        let language = if let Ok(l) = Language::try_from(value.get_language()) {
            l
        } else {
            println!("{}", "Invalid language. Setting default (en)".bright_yellow().bold());
            Language::English
        };

        let secret = value.get_secret().chars().collect();

        let tries = value.get_tries();

        Settings {
            language,
            secret,
            tries
        }
    }
}

pub fn get_settings<A: Args>(args: A) -> Settings {
    let mut settings = Settings::from(args);
    settings
}