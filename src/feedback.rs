use crate::char::*;

use colored::{ColoredString, Colorize};
use std::collections::{HashMap, HashSet};

pub struct Feedback {
    /// Contains the characters of the secret word as well as its status in each comparison
    secret: Vec<Char>,
    /// Contains the characters of the guess as well as its status in each comparison
    chars: Vec<Char>,
    /// Used to print the colorized word
    feedback_word: Vec<ColoredString>,
    /// Contains the abecedary with its chars colored to show its status
    abecedary: HashMap<char, ColoredString>,
    /// True if the guess is correct
    pub win: bool,
}

impl Feedback {
    pub fn new(secret_word: &[char], abecedary: &HashSet<char>) -> Self {
        let mut abc: HashMap<char, ColoredString> = HashMap::new();
        for c in abecedary.iter() {
            abc.insert(*c, ColoredString::from(c.to_uppercase().to_string()));
        }
        Self {
            secret: secret_word.iter().copied().map(Char::new).collect(),
            chars: Vec::new(),
            feedback_word: Vec::new(),
            abecedary: abc,
            win: false,
        }
    }

    pub fn get_secret(&self) -> String {
        self.secret.clone().into_iter().map(|x| x.chr).collect()
    }

    pub fn compare(&mut self, guess: String) {
        self.reset(guess);
        self.mark_correct();
        self.mark_misplaced();
        self.mark_incorrect();
    }

    fn mark_correct(&mut self) {
        self.win = true;
        for i in 0..self.chars.len() {
            if self.chars[i].chr == self.secret[i].chr {
                self.chars[i].set_correct();
                self.secret[i].set_correct();
                self.color_correct(i, self.chars[i].chr);
                continue;
            }
            self.win = false;
        }
    }

    fn mark_misplaced(&mut self) {
        for i in 0..self.chars.len() {
            if self.chars[i].has_status() {  // Checks if it's already correct
                continue;
            }
            for j in 0..self.secret.len() {
                if self.secret[j].has_status() {
                    continue;
                }
                if self.chars[i].chr == self.secret[j].chr {
                    self.chars[i].set_misplaced();
                    self.secret[j].set_misplaced();
                    self.color_misplaced(i, self.chars[i].chr);
                    break;
                }
            }
        }
    }

    fn mark_incorrect(&mut self) {
        for i in 0..self.chars.len() {
            if !self.chars[i].has_status() {
                self.color_incorrect(i, self.chars[i].chr);
            }
        }
    }

    fn reset(&mut self, new: String) {
        for c in self.secret.iter_mut() {
            c.set_incorrect();
        }
        self.win = true;
        self.chars = new.clone().chars().map(Char::new).collect();
        self.feedback_word = new
            .chars()
            .map(|x| ColoredString::from(x.to_uppercase().to_string()))
            .collect();
    }

    // Colorizing

    fn color_correct(&mut self, index: usize, char_in_abecedary: char) {
        self.colorize(index, char_in_abecedary, |x| x.bright_green())
    }

    fn color_misplaced(&mut self, index: usize, char_in_abecedary: char) {
        self.colorize(index, char_in_abecedary, |x| x.bright_yellow())
    }

    fn color_incorrect(&mut self, index: usize, char_in_abecedary: char) {
        self.colorize(index, char_in_abecedary, |x| x.red())
    }

    fn colorize(
        &mut self,
        index: usize,
        char_in_abecedary: char,
        function: fn(&str) -> ColoredString,
    ) {
        self.colorize_abecedary(char_in_abecedary, function);
        self.feedback_word[index] = function(&self.feedback_word[index]);
    }

    fn colorize_abecedary(&mut self, c: char, function: fn(&str) -> ColoredString) {
        self.abecedary
            .insert(c, function(self.abecedary.get(&c).unwrap()));
    }

    pub fn get_abecedary(&self) -> String {
        let mut s = String::new();
        let mut vec: Vec<(&char, &ColoredString)> = self.abecedary.iter().collect();
        vec.sort_by(|a, b| a.0.cmp(b.0));
        for i in vec {
            s = format!("{} {}", s, i.1);
        }
        s
    }
}

impl std::fmt::Display for Feedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in self.feedback_word.iter() {
            s = format!("{} {}", s, i);
        }
        f.write_str(&s)
    }
}
