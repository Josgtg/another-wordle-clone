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
    /// Contains all the words guessed
    feedback_history: Vec<String>,
    /// Contains the abecedary with its chars colored to show its status
    abecedary: HashMap<char, ColoredString>,
    colored_in_abc: HashSet<char>,
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
            feedback_history: Vec::new(),
            abecedary: abc,
            colored_in_abc: HashSet::new(),
            win: false,
        }
    }

    pub fn get_secret(&self) -> String {
        self.secret.clone().into_iter().map(|x| x.character).collect()
    }

    pub fn compare(&mut self, guess: String) {
        self.reset(guess);
        self.mark_correct();
        self.mark_misplaced();
        self.mark_incorrect();
        self.feedback_history.push(self.to_string());
    }

    fn mark_correct(&mut self) {
        self.win = true;
        for i in 0..self.chars.len() {
            if compare_chars(self.chars[i].character, self.secret[i].character) {
                self.chars[i].set_correct();
                self.secret[i].set_correct();
                if self.chars[i].character != self.secret[i].character {
                    // Marking correct the "Á" when user input had an "A" but the secret word an "Á"
                    self.color_correct(i, self.secret[i].character);
                } else {
                    self.color_correct(i, self.chars[i].character);
                }
                continue;
            }
            self.win = false;
        }
    }

    fn mark_misplaced(&mut self) {
        for i in 0..self.chars.len() {
            if self.chars[i].has_status() {
                // Checks if it's already correct
                continue;
            }
            for j in 0..self.secret.len() {
                if self.secret[j].has_status() {
                    continue;
                }
                if compare_chars(self.chars[i].character, self.secret[j].character) {
                    self.chars[i].set_misplaced();
                    self.secret[j].set_misplaced();
                    if self.chars[i].character != self.secret[j].character {
                        self.color_misplaced(i, self.secret[i].character);
                    } else {
                        self.color_misplaced(i, self.chars[i].character);
                    }
                    break;
                }
            }
        }
    }

    fn mark_incorrect(&mut self) {
        for i in 0..self.chars.len() {
            if !self.chars[i].has_status() {
                self.color_incorrect(i, self.chars[i].character);
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
        if !self.colored_in_abc.insert(c) {
            return;
        }
        self.abecedary
            .insert(c, function(self.abecedary.get(&c).unwrap()));
    }

    // Getters

    pub fn get_abecedary(&self) -> String {
        let mut s = String::new();
        let mut vec: Vec<(&char, &ColoredString)> = self.abecedary.iter().collect();
        vec.sort_by(|a, b| a.0.cmp(b.0));
        for i in vec {
            s = format!("{} {}", s, i.1);
        }
        s
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.feedback_history
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
