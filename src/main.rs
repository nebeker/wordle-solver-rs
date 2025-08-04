mod letter_guess;
mod word_guess;
mod word_picker;
mod dictionay_parser;

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
    let (actual_word, valid_words) = word_picker::word_picker::pick_word();
    println!("Guess the word:");
    let mut remaining_guesses = 5;
    while remaining_guesses > 0 {
        if handle_guess(&actual_word, &valid_words) { return; }
        remaining_guesses -= 1;
    }
    println!("The word was {}", actual_word);
}

fn handle_guess(actual_word: &String, valid_words: &Vec<String>) -> bool {
    let mut buffer = String::new();
    let stdin = io::stdin();
    _ = stdin.read_line(&mut buffer);
    let guess = WordGuess::from_str(buffer.trim());
    if guess.is_ok() {
        let mut guess = guess.unwrap();
        let is_valid = word_picker::word_picker::is_valid_word(guess.get_guessed_word(), &valid_words);
        if !is_valid
        {
            println!("Invalid word!");
        } else {
            let result = guess.check_word_guess(actual_word.as_str());
            println!("{}", guess);
            if result == WordGuessStatus::Correct
            {
                println!("Correct!");
                return true;
            }
        }
    }
    false
}
