mod letter_guess;
mod word_guess;
mod word_picker;

use std::io;
use std::str::FromStr;
use word_guess::word_guess::WordGuess;
use crate::letter_guess::letter_guess::LetterGuessStatus;
use crate::word_guess::word_guess::WordGuessStatus;

fn main() {
    println!("Legend: Incorret:{}, Incorrect position:{}, Correct:{}",
             LetterGuessStatus::Incorrect.to_string(),
             LetterGuessStatus::IncorrectPosition.to_string(),
             LetterGuessStatus::Correct.to_string());
    let actual_word = word_picker::word_picker::pick_word();
    println!("Guess the word:");
    let mut remaning_guesses = 5;
    while remaning_guesses > 0 {
        if handle_guess(&actual_word) { return; }
        remaning_guesses -= 1;
    }
}

fn handle_guess(actual_word: &String) -> bool {
    let mut buffer = String::new();
    let stdin = io::stdin();
    _ = stdin.read_line(&mut buffer);
    let guess = WordGuess::from_str(buffer.trim());
    if guess.is_ok() {
        let mut guess = guess.unwrap();
        let result = guess.check_word_guess(actual_word.as_str());
        println!("{}", guess);
        if result == WordGuessStatus::Correct
        {
            println!("Correct!");
            return true;
        }
    }
    false
}
