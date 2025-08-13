use std::path::Path;

pub mod dictionary_parser{
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
        let split_words: Vec<&str> = trimmed.split("/").collect();

        if let Some(result) = split_words.iter().next() {
            if result.len() == 5
            {
            return Some(result.to_string());
            }
        }
        None
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
        fn longer__word_returns_none(){
            let test_word = "Anglicanism/MS";
            let result = parse_word(test_word);
            assert_eq!(result.is_some(), false);
            assert_eq!(result.is_none(), true);
        }
    }
}