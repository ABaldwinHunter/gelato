use std::io;

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

    println!("Player one: what is your word?");
    let mut raw_secret = String::new();
    io::stdin().read_line(&mut raw_secret).expect("Failed to read line");
    let secret = raw_secret.trim();

    // clear screen
    print!("{}[2J", 27 as char);

    // println!("Shhh your secret is: {:?}", secret);

    println!("Player two: what is your guess?");
    loop {
        let mut raw_guess = String::new();
        io::stdin().read_line(&mut raw_guess).expect("Failed to read line");
        let guess = raw_guess.trim();

        if guess == secret {
            println!("Ya got it right!");
            break;
        } else {
            // determine number of letters correct
            let mut score = 0;
            for letter in secret.chars() {
                if guess.contains(letter) {
                    score += 1;
                }
            }
            println!("{} - {}", guess, score);
        }
    }
}
