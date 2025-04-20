pub mod letter_guess {
    use std::str::FromStr;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        pub letter: char,
        pub position: usize,
        pub status: LetterGuessStatus,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct LetterGuessParseError;
    impl FromStr for LetterGuess {
        type Err = LetterGuessParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.len() != 2 {
                return Err(LetterGuessParseError);
            }

            if let Some(letter) = s.chars().nth(1) {
                if let Some(status_char) = s.chars().nth(0) {
                    if let Ok(status) = LetterGuessStatus::from(status_char) {
                        return Ok(LetterGuess {
                            letter,
                            position: 0,
                            status,
                        });
                    }
                }
            }
            Err(LetterGuessParseError)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn create_letter_guess() {
            let mut letter_guess = LetterGuess::from_str("!B");
            assert!(letter_guess.is_ok());
            assert_eq!(letter_guess.unwrap().letter, 'B');
            letter_guess = LetterGuess::from_str("!B");
            assert_eq!(letter_guess.unwrap().status, LetterGuessStatus::Correct);
        }
    }
}
