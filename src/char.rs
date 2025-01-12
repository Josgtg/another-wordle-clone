#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharStatus {
    Correct,
    Misplaced,
    Incorrect,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Char {
    pub chr: char,
    pub status: CharStatus,
}

impl Char {
    pub fn new(c: char) -> Self {
        Self {
            chr: c,
            status: CharStatus::Incorrect,
        }
    }

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
