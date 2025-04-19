use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum LetterGuessStatus {
    Incorrect,
    Correct,
    IncorrectPosition,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct LetterGuess {
    letter: char,
    position: usize,
    status: Option<LetterGuessStatus>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WordGuessStatus {
    Incorrect,
    Correct,
}

#[derive(Debug, PartialEq)]
pub struct WordGuess {
    letter_guesses: [LetterGuess; 5],
    status: Option<WordGuessStatus>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseWordGuessError;

impl FromStr for WordGuess {
    type Err = ParseWordGuessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.len() != 5 || !s.is_ascii()
        {
            return Err(ParseWordGuessError);
        }
        let binding = s.to_ascii_uppercase();
        let letters = binding.chars();
        let mut new_letter_guesses : [LetterGuess; 5] = Default::default();

        for (index, l) in letters.enumerate() {
            if !l.is_alphabetic()
            {
                return Err(ParseWordGuessError);
            }
            new_letter_guesses[index] = LetterGuess{letter: l, position: index, status: None,}
        }

        Ok(WordGuess{
            letter_guesses: new_letter_guesses,
            status: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_word_guess() {
        let new_word_guess = WordGuess::from_str("abcde");
        assert_eq!(new_word_guess.unwrap().status, None);
        let new_word_guess = WordGuess::from_str("abcde");
        assert_eq!(new_word_guess.unwrap().letter_guesses[0].letter, 'A');
    }

    #[test]
    fn create_word_guess_too_long() {
        let new_word_guess = WordGuess::from_str("abcdef");
        assert!(new_word_guess.is_err());
    }


    #[test]
    fn create_word_guess_not_letter() {
        let new_word_guess = WordGuess::from_str("ab!de");
        assert!(new_word_guess.is_err());
    }
}