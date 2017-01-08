mod word_list;
use word_list::WordList;

use std::io;

fn count_overlap(secret: &str, guess: &str) -> usize {
    secret.chars().filter(|letter| guess.contains(*letter)).count()
}

fn main() {
    // accept word from player 1
    // extra: validate that letters unique
    // extra: validate that it's a real word
    // clear screen
    // player two plays game
    // mvp: accept guess, tell how many unique letters are correct
    // repeatedly
    // if user gets it right let them know
    // extras / part two: keep track of unique words guessed
    // display history to screen
    // super extra: evil game - tries to change word if you guess correctly, without violating
    // validity of history
    // AI that could guess strategically

    let words = WordList::new();

    println!("Player one: what is your word?");
    let mut raw_secret = String::new();
    io::stdin().read_line(&mut raw_secret).expect("Failed to read line");
    let secret = raw_secret.trim();

    if words.invalid_input(secret) {
        panic!("No bueno input!!!");
    }

    // clear screen
    print!("{}[2J", 27 as char);

    println!("Player two: what is your guess?");
    loop {
        let mut raw_guess = String::new();
        io::stdin().read_line(&mut raw_guess).expect("Failed to read line");
        let guess = raw_guess.trim();

        if guess == secret {
            println!("Ya got it right!");
            break;
        } else if words.invalid_input(guess) {
            println!("Bad input!");
            continue;
        } else {
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
