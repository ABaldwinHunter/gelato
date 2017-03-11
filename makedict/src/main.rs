use std::env;
use std::io::prelude::*;
use std::fs::File;

struct Expander {
}

impl Expander {
    fn via(word: &str) -> Vec<String> {
        // TODO: make this smarter
        // make two functions - pluralize and conjugate
        vec![
            word.to_owned(),
            format!("{}s", word),
            format!("{}ing", word),
            format!("{}ed", word),
        ]
    }

    fn pluralize(word: &str) -> String {
        word.to_owned()
    }
}

#[test]
fn test_expander() {
    assert_eq!(Expander::via("dog"), vec!["dog", "dogs", "doging", "doged"]);
    assert_eq!(Expander::via("walk"), vec!["walk", "walks", "walking", "walked"]);
}

#[test]
fn test_expander_pluralize() {
    // works for nouns and verbs indiscriminately
    // assumes none of our inputs are already pluralized

    // ends in a consonant
    assert_eq!(Expander::pluralize("dog"), "dogs");
    assert_eq!(Expander::pluralize("walk"), "walks");
    // ends in a vowel that's not y
    assert_eq!(Expander::pluralize("stigma"), "stimgas");
    assert_eq!(Expander::pluralize("name"), "names");

    // ends in  consonant followed by y
    assert_eq!(Expander::pluralize("sky"), "skies");
    // ends in  vowel followed by y
    assert_eq!(Expander::pluralize("pray"), "prays");
    // ends in i
    assert_eq!(Expander::pluralize("ski"), "skis");
    // ends in sh or ch
    assert_eq!(Expander::pluralize("fish"), "fishes");
    assert_eq!(Expander::pluralize("church"), "churches");
    // ends in z
    assert_eq!(Expander::pluralize("quiz"), "quizzes");
    // ends in ss
    assert_eq!(Expander::pluralize("glass"), "glasses");
    // doesn't end in ss, but ends in s
    assert_eq!(Expander::pluralize("gas"), "gasses");

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

    let mut output = String::new();

    for word in words.into_iter().filter(|word| word.len() == 5) {
        output.push_str(&word);
        output.push_str("\n");
    }

    let mut computed_file = File::create(computed_dict_path).unwrap();

    computed_file.write_all(output.as_bytes()).unwrap();

    println!("Updated {}", computed_dict_path);
}
