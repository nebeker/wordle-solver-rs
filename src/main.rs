mod letter_guess;
mod word_guess;

use std::str::FromStr;
use word_guess::word_guess::WordGuess;

fn main() {
    let guess = WordGuess::from_str("abcde");
    println!("Your Guess is {}", guess.unwrap());

    let guess_with_stats = WordGuess::from_str("xA!B?C_DxE");
    println!("Your Guess with status is {}", guess_with_stats.unwrap());

    let guess_status = WordGuess::from_str("abcde").unwrap().check_word_guess("abcde");
    print!("Your Guess status is {:?}", guess_status);
}
