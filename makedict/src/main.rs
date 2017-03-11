use std::env;
use std::io::prelude::*;
use std::fs::File;

struct Expander {
}

impl Expander {
    fn via(word: &str) -> Vec<String> {
        // TODO: make this smarter
        vec![
            word.to_owned(),
            format!("{}s", word),
            format!("{}ing", word),
            format!("{}ed", word),
        ]
    }
}

#[test]
fn test_expander() {
    assert_eq!(Expander::via("dog"), vec!["dog", "dogs", "doging", "doged"]);
    assert_eq!(Expander::via("walk"), vec!["walk", "walks", "walking", "walked"]);
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

    let mut words: Vec<String> = vec![];

    for word in s.lines() {
        if &word == &word.to_lowercase() {
            for expanded_word in Expander::via(&word) {
                words.push(expanded_word);
            }
        }
    }

    println!("{:?}", words);

    // TODO: write to the output file
}
