pub mod dictionary_parser{
    use std::collections::HashMap;
    use std::fs::File;
    use std::io;
    use std::io::BufRead;
    use std::path::Path;

    pub fn parse_file_for_words(path: &str) -> Result<Vec<String>, io::Error> {
        let mut words: Vec<String> = Vec::new();
        let mut word_count = 0;
        let mut parsed_word_count = 0;
        if let Ok(lines) = read_lines(path) {
            for line in lines.map_while(Result::ok) {
                if let Some(cleaned) = parse_word(&*line) {
                    words.push(cleaned);
                parsed_word_count += 1;
                }
                    word_count += 1;
            }
        }
        println!("Parsed {} words in {} words", parsed_word_count, word_count);
        Ok(words)
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn parse_word(word: &str) -> Option<String>{
        let trimmed = word.trim().to_ascii_lowercase().chars().filter(
            |c: &char| { *c != '\'' && !c.is_numeric() }
        ).collect::<String>();

        if let Some(result) = trimmed.split("/").next() {
            if result.len() == 5
            {
            return Some(result.to_string());
            }
        }
        None
    }

    #[derive(Debug)]
    pub struct LetterFrequencies
    {
        pub occurrences: HashMap<char, u32>,
        pub total_occurrences: u32
    }

    impl LetterFrequencies {
        pub fn new() -> LetterFrequencies {
            let occurrences = HashMap::new();
            LetterFrequencies { occurrences: occurrences, total_occurrences: 0 }
        }

        pub fn add_word_to_letter_frequencies(&mut self, word: String) {
            for letter in word.chars() {
                self.occurrences.insert(letter, self.occurrences.get(&letter).unwrap_or(&0) + 1);
            }
            self.total_occurrences += word.len() as u32;
        }

        pub fn to_string(&self) -> String {
            let mut letter_frequencies = String::new();
            for (letter, occurrences) in &self.occurrences {
                letter_frequencies.push(*letter);
                letter_frequencies.push(':');
                letter_frequencies.push(' ');
                for char in occurrences.to_string().chars() {
                    letter_frequencies.push(char);
                }
                letter_frequencies.push(';');
            }
            letter_frequencies
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn five_letter_with_meta_word_returns(){
            let test_word = "Agata/M";
            let result = parse_word(test_word);
            assert_eq!(result.is_some(), true);
            assert_eq!(result.unwrap(), "agata");
        }

        #[test]
        fn five_letter_with_no_meta_word_returns(){
            let test_word = "Canad";
            let result = parse_word(test_word);
            assert_eq!(result.is_some(), true);
            assert_eq!(result.unwrap(), "canad");
        }

        #[test]
        fn five_letter_with_apostrophe_word_returns(){
            let test_word = "D'Arcy";
            let result = parse_word(test_word);
            assert_eq!(result.is_some(), true);
            assert_eq!(result.unwrap(), "darcy");
        }

        #[test]
        fn longer_word_returns_none(){
            let test_word = "Anglicanism/MS";
            let result = parse_word(test_word);
            assert_eq!(result.is_some(), false);
            assert_eq!(result.is_none(), true);
        }

        #[test]
        fn add_word_to_letter_frequencies_apple_four_letters() {
            let mut letter_frequencies = LetterFrequencies::new();
            letter_frequencies.add_word_to_letter_frequencies(String::from("apple"));

            assert_eq!(letter_frequencies.total_occurrences, 5);
            assert_eq!(letter_frequencies.occurrences.len(), 4);
        }

        #[test]
        fn add_word_to_letter_frequencies_multiple_apple_four_letters() {
            let mut letter_frequencies = LetterFrequencies::new();
            letter_frequencies.add_word_to_letter_frequencies(String::from("apple"));
            letter_frequencies.add_word_to_letter_frequencies(String::from("apple"));

            assert_eq!(letter_frequencies.total_occurrences, 10);
            assert_eq!(letter_frequencies.occurrences.len(), 4);
        }

        #[test]
        fn add_word_to_letter_frequencies_multiple_words_four_e() {
            let mut letter_frequencies = LetterFrequencies::new();
            letter_frequencies.add_word_to_letter_frequencies(String::from("apple"));
            letter_frequencies.add_word_to_letter_frequencies(String::from("fence"));
            letter_frequencies.add_word_to_letter_frequencies(String::from("fires"));

            assert_eq!(letter_frequencies.total_occurrences, 15);
            assert_eq!(letter_frequencies.occurrences.get(&'e').unwrap(), &4);
        }

        #[test]
        fn add_multiple_words_to_string(){
            let mut letter_frequencies = LetterFrequencies::new();
            letter_frequencies.add_word_to_letter_frequencies(String::from("apple"));
            letter_frequencies.add_word_to_letter_frequencies(String::from("fence"));
            letter_frequencies.add_word_to_letter_frequencies(String::from("fires"));

            assert_eq!(letter_frequencies.to_string().len(), 5*10);
        }
    }
}