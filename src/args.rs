use crate::language::{Language, LanguagePack};
use clap::Parser;
use std::str::FromStr;

const DEFAULT_LANG: Language = Language::Spanish;
const DEFAULT_SECRET: Option<Vec<char>> = None;
const DEFAULT_TRIES: u8 = 5;

#[derive(Debug, Parser)]
pub struct Args {
    /// Change the default language | Cambia el idioma por defecto ("en", "es")
    #[arg(short = 'l', long = "language")]
    language: Option<String>,
    /// Set a secret word | Asigna una palabra secreta
    #[arg(short = 's', long = "secret")]
    secret: Option<String>,
    /// Set max guesses | Asigna los intentos máximos
    #[arg(short = 't', long = "tries")]
    tries: Option<u8>,
}

impl Args {
    pub fn get_language(&self) -> Language {
        if let Some(language_str) = &self.language {
            if let Ok(language) = Language::from_str(language_str) {
                return language;
            }
        }
        DEFAULT_LANG
    }

    pub fn get_secret(&self, abecedary: &mut std::collections::HashSet<char>, lang: &LanguagePack) -> Option<Vec<char>> {
        let secret_vec: Vec<char> = if let Some(secret) = &self.secret {
            secret.chars().collect()
        } else {
            return DEFAULT_SECRET;
        };

        if secret_vec.len() == 5 {
            for c in secret_vec.iter() {
                abecedary.insert(c.clone());
            }
            Some(secret_vec)
        } else {
            eprintln!("{}", lang.args_invalid_secret_word);
            DEFAULT_SECRET
        }
    }

    pub fn get_tries(&self) -> u8 {
        if let Some(tries) = self.tries {
            tries
        } else {
            DEFAULT_TRIES
        }
    }
}
