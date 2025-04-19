use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum LetterGuessStatus {
    Incorrect,
    Correct,
    IncorrectPosition,
    Unevaluated,
}

impl Default for LetterGuessStatus {
    fn default() -> Self {
        LetterGuessStatus::Unevaluated
    }
}

impl LetterGuessStatus {
    pub fn from(c: char) -> Result<LetterGuessStatus, ()> {
        match c {
            '?' => Ok(LetterGuessStatus::Unevaluated),
            '_' => Ok(LetterGuessStatus::IncorrectPosition),
            'x' => Ok(LetterGuessStatus::Incorrect),
            '!' => Ok(LetterGuessStatus::Correct),
            _ => Err(()),
        }
    }
}

impl ToString for LetterGuessStatus {
    fn to_string(&self) -> String {
        match self {
            LetterGuessStatus::Unevaluated => String::from("?"),
            LetterGuessStatus::IncorrectPosition => String::from("_"),
            LetterGuessStatus::Incorrect => String::from("x"),
            LetterGuessStatus::Correct => String::from("!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct LetterGuess {
    letter: char,
    position: usize,
    status: LetterGuessStatus,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LetterGuessParseError;
impl FromStr for LetterGuess {
    type Err = LetterGuessParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2
        {
            return Err(LetterGuessParseError);
        }

        if let Some(letter) = s.chars().nth(1)
        {
            if let Some(status_char) = s.chars().nth(0)
            {
                if let Ok(status) = LetterGuessStatus::from(status_char)
                {
                    return Ok(LetterGuess{letter, position: 0, status});
                }
            }
        }
        Err(LetterGuessParseError)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum WordGuessStatus {
    Incorrect,
    Correct,
}

#[derive(Debug, PartialEq)]
pub struct WordGuess {
    pub letter_guesses: [LetterGuess; 5],
    pub status: Option<WordGuessStatus>,
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
            new_letter_guesses[index] = LetterGuess{letter: l, position: index, status: LetterGuessStatus::Unevaluated,}
        }

        Ok(WordGuess{
            letter_guesses: new_letter_guesses,
            status: None,
        })
    }
}

impl fmt::Display for WordGuess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut formatted_word_guess= "".to_owned();
        for letter in self.letter_guesses.iter() {
            formatted_word_guess.push_str(letter.status.to_string().as_str());
            formatted_word_guess.push(letter.letter);
        }
        write!(f, "{}", formatted_word_guess)
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

    #[test]
    fn create_letter_guess() {
        let mut letter_guess = LetterGuess::from_str("!B");
        assert!(letter_guess.is_ok());
        assert_eq!(letter_guess.unwrap().letter, 'B');
        letter_guess = LetterGuess::from_str("!B");
        assert_eq!(letter_guess.unwrap().status, LetterGuessStatus::Correct);
    }
}