use colored::ColoredString;

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
            colored: ColoredString::from(c.to_string()),
            status: CharStatus::Incorrect,
        }
    }

    /// Returns true if the status is not incorrect
    pub fn has_status(&self) -> bool {
        self.status != CharStatus::Incorrect
    }

    pub fn set_correct(&mut self) {
        self.status = CharStatus::Correct
    }

    pub fn set_misplaced(&mut self) {
        self.status = CharStatus::Misplaced
    }

    pub fn set_incorrect(&mut self) {
        self.status = CharStatus::Incorrect
    }
}

pub fn contains_utf8_only(s: &str) -> bool {
    s.contains('á')
        || s.contains('é')
        || s.contains('í')
        || s.contains('ó')
        || s.contains('ú')
        || s.contains('ü')
}

pub fn asciify(c: char) -> char {
    match c {
        'á' => 'a',
        'é' => 'e',
        'í' => 'i',
        'ó' => 'o',
        'ú' | 'ü' => 'u',
        default => default,
    }
}

pub fn asciify_str(s: &str) -> String {
    let mut ascii = String::new();
    s.chars().for_each(|c| ascii.push(asciify(c)));
    ascii
}

/// Checks if a is similar to b
pub fn compare_chars(a: char, b: char) -> bool {
    a == asciify(b) || a == b
}
