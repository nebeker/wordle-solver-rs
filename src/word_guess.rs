pub mod word_guess {
    use std::fmt;
    use std::str::FromStr;
    use crate::letter_guess::letter_guess::{LetterGuess, LetterGuessStatus};

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum WordGuessStatus {
        Incorrect,
        Correct,
        Unevaluated,
    }

    #[derive(Debug, PartialEq)]
    pub struct WordGuess {
        pub letter_guesses: [LetterGuess; 5],
        pub status: WordGuessStatus,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct ParseWordGuessError;

    impl FromStr for WordGuess {
        type Err = ParseWordGuessError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if !s.is_ascii() {
                return Err(ParseWordGuessError);
            }

            match s.len() {
                5 => {
                    let new_letter_guesses = match Self::parse_letters(s) {
                        Ok(value) => value,
                        Err(value) => return value,
                    };
                    Ok(WordGuess {
                        letter_guesses: new_letter_guesses,
                        status: WordGuessStatus::Unevaluated,
                    })
                }
                10 => {
                    let new_letter_guesses = match Self::parse_letters_with_status(s) {
                        Ok(value) => value,
                        Err(value) => return value,
                    };
                    Ok(WordGuess {
                        letter_guesses: new_letter_guesses,
                        status: WordGuessStatus::Unevaluated,
                    })
                }
                _ => return Err(ParseWordGuessError),
            }
        }
    }

    impl WordGuess {
        fn parse_letters(
            s: &str,
        ) -> Result<[LetterGuess; 5], Result<WordGuess, <WordGuess as FromStr>::Err>> {
            let binding = s.to_ascii_uppercase();
            let letters = binding.chars();
            let mut new_letter_guesses: [LetterGuess; 5] = Default::default();

            for (index, l) in letters.enumerate() {
                if !l.is_alphabetic() {
                    return Err(Err(ParseWordGuessError));
                }
                new_letter_guesses[index] = LetterGuess {
                    letter: l,
                    position: index,
                    status: LetterGuessStatus::Unevaluated,
                }
            }
            Ok(new_letter_guesses)
        }

        fn parse_letters_with_status(
            s: &str,
        ) -> Result<[LetterGuess; 5], Result<WordGuess, <WordGuess as FromStr>::Err>> {
            let mut new_letter_guesses: [LetterGuess; 5] = Default::default();
            let binding = s.chars().collect::<Vec<char>>();
            let letter_pairs = binding.chunks(2);

            for (index, l) in letter_pairs.enumerate() {
                let first_letter = l[0];
                let second_letter = l[1].to_ascii_uppercase();
                if !second_letter.is_alphabetic() {
                    return Err(Err(ParseWordGuessError));
                }
                let pair = first_letter.to_string() + second_letter.to_string().as_str();
                if let Ok(mut guess) = LetterGuess::from_str(pair.as_str()) {
                    guess.position = index;
                    new_letter_guesses[index] = guess;
                } else {
                    return Err(Err(ParseWordGuessError));
                }
            }
            Ok(new_letter_guesses)
        }

        pub fn check_word_guess(&mut self, word: &str) -> WordGuessStatus {
            let binding = word.to_ascii_uppercase();
            let mut actual_letters = binding.chars();
            let actual_letter_vec: Vec<char> = actual_letters.clone().collect();

            for index in 0..self.letter_guesses.len() {
                if self.letter_guesses[index].letter == actual_letter_vec[index] {
                    self.letter_guesses[index].status = LetterGuessStatus::Correct;
                } else if actual_letters.any(|l| l == self.letter_guesses[index].letter) {
                    self.letter_guesses[index].status = LetterGuessStatus::IncorrectPosition;
                } else {
                    self.letter_guesses[index].status = LetterGuessStatus::Incorrect;
                }
            }

            self.status = if self
                .letter_guesses
                .iter()
                .all(|g| g.status == LetterGuessStatus::Correct)
            {
                WordGuessStatus::Correct
            } else {
                WordGuessStatus::Incorrect
            };

            return self.status.clone();
        }
    }

    impl fmt::Display for WordGuess {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut formatted_word_guess = "".to_owned();
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
            assert_eq!(new_word_guess.unwrap().status, WordGuessStatus::Unevaluated);
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
        fn check_correct_word_guess() {
            let check_result = WordGuess::from_str("abcde")
                .unwrap()
                .check_word_guess("abcde");
            assert_eq!(check_result, WordGuessStatus::Correct);
        }

        #[test]
        fn check_incorrect_word_guess() {
            let mut word_guess = WordGuess::from_str("acbde").unwrap();
            let check_result = word_guess.check_word_guess("abcde");
            assert_eq!(check_result, WordGuessStatus::Incorrect);
            assert_eq!(word_guess.letter_guesses[1].status, LetterGuessStatus::IncorrectPosition);
            assert_eq!(word_guess.letter_guesses[2].status, LetterGuessStatus::IncorrectPosition);
        }
    }
}
