mod word_list;
use word_list::WordList;

use std::io;

fn count_overlap(secret: &str, guess: &str) -> usize {
    secret.chars().filter(|letter| guess.contains(*letter)).count()
}

fn main() {
    let words = WordList::new();

    let secret = words.sample();

    let mut guess_count = 0;

    println!("OK, thought of a secret word. From a list of {} words.", words.count());
    println!("What is your guess?");
    loop {
        let mut raw_guess = String::new();
        io::stdin().read_line(&mut raw_guess).expect("Failed to read line");
        let guess = raw_guess.trim();

        if guess == secret {
            guess_count += 1;
            println!("Ya got it right, in {} guesses!", guess_count);
            break;
        } else if words.invalid_input(guess) {
            println!("Bad input!");
            continue;
        } else {
            guess_count += 1;
            println!("{} - {}", guess, count_overlap(secret, guess));
        }
    }
}

#[cfg(test)]
mod tests {
    use count_overlap;

    #[test]
    fn test_count_overlap() {
        assert_eq!(count_overlap("night", "light"), 4);
    }

    #[test]
    fn test_count_overlap_duplicate_letters() {
        assert_eq!(count_overlap("night", "nnnnn"), 1);
    }
}
