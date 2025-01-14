use clap::Parser;

use crate::language::LanguagePack;

// FIXME: I'm really sure there are better ways to do this

pub trait Args {
    fn get_language(&self) -> String;
    fn get_secret(&self) -> String;
    fn get_tries(&self) -> u32;
}

#[derive(Debug, Parser)]
pub struct ArgsEn {
    #[arg(default_value="en", help=LanguagePack::english().cli_language_help)]
    pub language: String,
    #[arg(default_value="", help=LanguagePack::english().cli_secret_help)]
    pub secret: String,
    #[arg(default_value="5", help=LanguagePack::english().cli_tries_help)]
    pub tries: u32
}

#[derive(Debug, Parser)]
pub struct ArgsEs {
    #[arg(default_value="es", help=LanguagePack::spanish().cli_language_help)]
    pub language: String,
    #[arg(default_value="", help=LanguagePack::spanish().cli_secret_help)]
    pub secret: String,
    #[arg(default_value="5", help=LanguagePack::spanish().cli_tries_help)]
    pub tries: u32
}


impl Args for ArgsEn {
    fn get_language(&self) -> String {
        self.language.clone()
    }

    fn get_secret(&self) -> String {
        self.secret.clone()
    }

    fn get_tries(&self) -> u32 {
        self.tries
    }
}

impl Args for ArgsEs {
    fn get_language(&self) -> String {
        self.language.clone()
    }

    fn get_secret(&self) -> String {
        self.secret.clone()
    }

    fn get_tries(&self) -> u32 {
        self.tries
    }
}