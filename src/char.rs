use std::hash::Hash;

use colored::{ColoredString, Colorize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharStatus {
    Correct,
    Misplaced,
    Incorrect,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Char {
    pub character: char,
    pub colored: ColoredString,
    pub status: CharStatus,
}

impl Char {
    pub fn new(c: char) -> Self {
        Self {
            character: c,
            colored: ColoredString::from(c.to_uppercase().to_string()),
            status: CharStatus::Incorrect,
        }
    }

    pub fn is_incorrect(&self) -> bool {
        self.status == CharStatus::Incorrect
    }

    /// Sets status given and colors the char to the corresponding color
    pub fn set_status(&mut self, status: CharStatus) {
        self.colored.clear_style();
        self.colored = match &status {
            CharStatus::Correct => self.colored.clone().bright_green(),
            CharStatus::Misplaced => self.colored.clone().bright_yellow(),
            CharStatus::Incorrect => self.colored.clone().bright_red(),
        };
        self.status = status;
    }
}

impl Hash for Char {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.character.hash(state);
    }
}

impl PartialOrd for Char {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.character.partial_cmp(&other.character)
    }
}
impl Ord for Char {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.character.cmp(&other.character)
    }
}
