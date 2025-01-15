use crate::char::{*, CharStatus::*};
use crate::ascii::compare_chars;
use std::collections::{HashSet, HashMap};

pub struct Feedback {
    /// Contains the characters of the secret word
    secret: Vec<Char>,
    /// Contains the characters of the guess
    guess: Vec<Char>,
    /// Contains all the words guessed
    feedback_history: Vec<String>,
    /// Contains the abecedary with its chars colored to show its status
    abecedary: HashMap<char, Char>,
    /// True if the guess is correct
    pub win: bool,
}

impl Feedback {
    pub fn new(secret_word: &[char], abecedary: &HashSet<char>) -> Self {
        Self {
            secret: secret_word.iter().copied().map(Char::new).collect(),
            guess: Vec::new(),
            feedback_history: Vec::new(),
            abecedary: abecedary.iter().map(|c| (*c, Char::new(*c))).collect(),
            win: false,
        }
    }

    fn reset(&mut self, new: String) {
        self.secret.iter_mut().for_each(|c| c.status = Incorrect);
        self.guess = new.chars().map(Char::new).collect();
        self.win = true;
    }

    pub fn compare(&mut self, guess: String) {
        self.reset(guess);
        self.mark_correct();
        self.mark_misplaced();
        self.mark_incorrect();
        self.feedback_history.push(self.get_feedback());
    }

    fn color_abecedary(&mut self, letter: char, color: CharStatus) {
        self.abecedary.get_mut(&letter).unwrap().set_status(color);
    }

    fn mark_correct(&mut self) {
        self.win = true;
        for i in 0..self.guess.len() {
            if compare_chars(self.guess[i].character, self.secret[i].character) {
                self.guess[i].set_status(Correct);
                self.secret[i].set_status(Correct);
                if self.guess[i].character != self.secret[i].character {
                    // If this is true, it means the actual letter was the one in the secret word
                    // In guess: "a" - In secret: "á", so "á" must be set as correct, not "a"
                    self.color_abecedary(self.secret[i].character, Correct);
                } else {
                    self.color_abecedary(self.guess[i].character, Correct);
                }
                continue;
            }
            self.win = false;
        }
    }

    fn mark_misplaced(&mut self) {
        for i in 0..self.guess.len() {
            if !self.guess[i].is_incorrect() {
                // Checks if it's already correct or misplaced
                continue;
            }
            for j in 0..self.secret.len() {
                if !self.secret[j].is_incorrect() {
                    continue;
                }
                if compare_chars(self.guess[i].character, self.secret[j].character) {
                    self.guess[i].set_status(Misplaced);
                    self.secret[j].set_status(Misplaced);
                    if self.guess[i].character != self.secret[j].character {
                        self.color_abecedary(self.secret[j].character, Misplaced);
                    } else {
                        self.color_abecedary(self.guess[i].character, Misplaced);
                    }
                    break;  // Must break to avoid marking all instances of self.guess[i] in secret word as misplaced
                }
            }
        }
    }

    fn mark_incorrect(&mut self) {
        for i in 0..self.guess.len() {
            if self.guess[i].is_incorrect() {
                self.guess[i].set_status(Incorrect);
                self.color_abecedary(self.guess[i].character, Incorrect);
            }
        }
    }

    // Getters

    pub fn get_secret(&self) -> String {
        self.secret.iter().fold(String::new(), |s, c| format!("{}{}", s, c.character))
    }

    pub fn get_feedback(&self) -> String {
        self.guess.iter().fold(String::new(), |s, c| format!("{} {}", s, c.colored))
    }

    pub fn get_abecedary(&self) -> String {
        let mut abecedary_vec: Vec<Char> = self.abecedary.clone().into_iter().map(|x| x.1).collect();
        abecedary_vec.sort();
        abecedary_vec.into_iter().fold(String::new(),|s, c| format!("{} {}", s, c.colored))
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.feedback_history
    }
}
