use std::env;
use std::io::prelude::*;
use std::fs::File;

struct Expander {
}

impl Expander {
    fn via(word: &str) -> Vec<&str> {
        vec!["hello"]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let full_dict_path = &args[1];
    let computed_dict_path = &args[2];

    println!("full_dict_path: {}", full_dict_path);
    println!("computed_dict_path: {}", computed_dict_path);

    let mut f = File::open(full_dict_path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    // println!("Input dictionary contents: {:?}", s);

    let mut words: Vec<&str> = vec![];

    for word in s.lines() {
        // if !&word.capitalized?
        if &word == &word.to_lowercase() {
            words.push(&word);
            for expanded_word in Expander::via(&word) {
                words.push(expanded_word);
            }
        }
    }

    println!("{:?}", words);
}
