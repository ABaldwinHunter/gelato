use std::io::prelude::*;
use std::fs::File;
extern crate rand;
use self::rand::distributions::{IndependentSample, Range};

pub struct WordList {
    words: Vec<String>,
}

impl WordList {
    // TODO: return a Result?
    pub fn new() -> WordList {
        let mut f = File::open("/usr/share/dict/words").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let mut words = vec![];

        for word in s.lines() {
            if WordList::valid(word) {
                words.push(word.to_owned());
            }
        }

        WordList {
            words: words,
        }
    }

    pub fn invalid_input(&self, other_word: &str) -> bool {
        !self.words.iter().any(|word| word == other_word)
    }

    pub fn sample(&self) -> &str {
        let between = Range::new(0, self.words.len());
        let mut rng = rand::thread_rng();
        let index = between.ind_sample(&mut rng);
        &self.words[index]
    }

    fn valid(word: &str) -> bool {
        word.chars().count() == 5 && char::is_lowercase(word.chars().nth(0).unwrap())
    }
}
