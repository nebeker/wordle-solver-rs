pub enum LetterGuessStatus {
    Incorrect,
    Correct,
    IncorrectPosition,
}

pub struct LetterGuess {
    letter: char,
    position: usize,
    status: Option<LetterGuessStatus>,
}

pub enum WordGuessStatus {
    Incorrect,
    Correct,
}

pub struct WordGuess {
    letter_guesses: [LetterGuess; 5],
    status: Option<WordGuessStatus>,
}
