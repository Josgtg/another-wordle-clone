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
