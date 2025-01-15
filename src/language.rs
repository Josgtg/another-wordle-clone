use std::str::FromStr;

use strum_macros::EnumIter;

#[derive(PartialEq, Eq, EnumIter)]
pub enum Language {
    English,
    Spanish,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Language::English => "English",
            Language::Spanish => "Español",
        })
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "english" | "en" => Ok(Language::English),
            "español" | "es" => Ok(Language::Spanish),
            _ => Err(()),
        }
    }
}

pub struct LanguagePack<'a> {
    pub welcome: &'a str,
    pub commands: &'a str,
    pub win: &'a str,
    pub loss: &'a str,
    pub tries_left: &'a str,
    pub err_invalid_chars: &'a str,
    pub err_invalid_len: &'a str,
    pub err_invalid_word: &'a str,
    pub prompt_guess_title: &'a str,
    pub prompt_language_title: &'a str,
    pub prompt_play_again: &'a str,
    pub no_word_guessed: &'a str,
}

impl LanguagePack<'static> {
    pub const fn english() -> LanguagePack<'static> {
        LanguagePack {
            welcome: WELCOME_EN,
            commands: COMMANDS_EN,
            win: WIN_EN,
            loss: LOSS_EN,
            tries_left: TRIES_LEFT_EN,
            err_invalid_chars: ERR_INVALID_CHARS_EN,
            err_invalid_len: ERR_INVALID_LEN_EN,
            err_invalid_word: ERR_INVALID_WORD_EN,
            prompt_guess_title: PROMPT_GUESS_TITLE_EN,
            prompt_language_title: PROMPT_LANGUAGE_TITLE_EN,
            prompt_play_again: PROMPT_PLAY_AGAIN_EN,
            no_word_guessed: NO_WORD_GUESSED_EN,
        }
    }

    pub const fn spanish() -> LanguagePack<'static> {
        LanguagePack {
            welcome: WELCOME_ES,
            commands: COMMANDS_ES,
            win: WIN_ES,
            loss: LOSS_ES,
            tries_left: TRIES_LEFT_ES,
            err_invalid_chars: ERR_INVALID_CHARS_ES,
            err_invalid_len: ERR_INVALID_LEN_ES,
            err_invalid_word: ERR_INVALID_WORD_ES,
            prompt_guess_title: PROMPT_GUESS_TITLE_ES,
            prompt_language_title: PROMPT_LANGUAGE_TITLE_ES,
            prompt_play_again: PROMPT_PLAY_AGAIN_ES,
            no_word_guessed: NO_WORD_GUESSED_ES,
        }
    }
}

// Constants

// Game

const WELCOME_EN: &str = "\
Welcome! This is a simple game of discovering a 5 letter secret word. If you're not familiar to the rules they're the next:\n
When you write a word, the letters of the word will be shown in different colors. Those colors mean the next:
\x1b[92mGreen\x1b[0m: The letter is in the secret word in that position
\x1b[93mYellow\x1b[0m: The letter is in the secret word, but in another position
\x1b[91mRed\x1b[0m: The letter is not in the secret word\n
You can enter \"h\" when writing a word to see useful commands\n";
const WELCOME_ES: &str = "\
Este es un juego en el que tienes que adivinar una palabra secreta de 5 letras. Si no conoces las reglas, son las siguientes:\n
Cuando escribas una palabra, sus letras serán mostradas en diferentes colores. Los colores significan lo siguiente:
\x1b[92mVerde\x1b[0m: La letra está en la palabra secreta en esa posición
\x1b[93mAmarillo\x1b[0m: La letra está en la palabra secreta, pero en otra posición
\x1b[91mRojo\x1b[0m: La letra no está en la palabra secreta\n
Puedes escribir \"h\" cuando estés ingresando una palabra para consultar comandos útiles\n";

const COMMANDS_EN: &str = "\
w: Show previous guessed word
l: Show all the words guessed at that point
a: Show all possible letters, like printing the abecedary, with revealed hints
t: Show how many tries are left
h: Show available commands
q: Quits the game";
const COMMANDS_ES: &str = "\
w: Muestra la palabra ingresada anteriormente
l: Muestra todas las palabras ingresadas hasta ese punto
a: Muestra todas las letras disponibles, como el abecedario, con las pistas reveladas
t: Muestra los intentos restantes
h: Muestra los comandos disponibles
q: Sale del juego";

const WIN_EN: &str = "Congratulations! You win!";
const WIN_ES: &str = "Has acertado la palabra ¡Felicidades!";

const LOSS_EN: &str = "You loose! Word was";
const LOSS_ES: &str = "¡Perdiste! La palabra era";

const TRIES_LEFT_EN: &str = "Tries left";
const TRIES_LEFT_ES: &str = "Intentos restantes";

const ERR_INVALID_CHARS_EN: &str = "Guess contains invalid characters";
const ERR_INVALID_CHARS_ES: &str = "La palabra contiene caracteres inválidos";

const ERR_INVALID_LEN_EN: &str = "Guess must be 5 characters long";
const ERR_INVALID_LEN_ES: &str = "La palabra debe contener 5 caracteres";

const ERR_INVALID_WORD_EN: &str = "Unrecognized word. Try another one";
const ERR_INVALID_WORD_ES: &str = "La palabra no ha sido encontrada. Intenta una nueva";

const PROMPT_GUESS_TITLE_EN: &str = "Write your guess:";
const PROMPT_GUESS_TITLE_ES: &str = "Escribe una palabra:";

const PROMPT_LANGUAGE_TITLE_EN: &str = "Select language:";
const PROMPT_LANGUAGE_TITLE_ES: &str = "Selecciona idioma:";

const PROMPT_PLAY_AGAIN_EN: &str = "Play again?";
const PROMPT_PLAY_AGAIN_ES: &str = "¿Quires jugar otra vez?";

const NO_WORD_GUESSED_EN: &str = "No word has been guessed yet";
const NO_WORD_GUESSED_ES: &str = "No se ha ingresado ninguna palabra todavía";
