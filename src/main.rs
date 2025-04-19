use std::str::FromStr;
use wordle_solver_rs::{WordGuess};

fn main() {
    let _guess = WordGuess::from_str("abcde");
    println!("Your Guess is {}", _guess.unwrap());
}
