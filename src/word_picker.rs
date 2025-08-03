pub mod word_picker {
    use rand::prelude::IndexedRandom;
    use crate::dictionay_parser::dictionary_parser;

    pub fn pick_word() ->(String,Vec<String>){
        let mut rng = rand::rng();
        if let Ok(words) = dictionary_parser::parse_file_for_words("./en-US.dic")
        {
            return (words.choose(&mut rng).unwrap().to_string(),words);
        }
        println!("Failed to read dictionary - using sample word list");
        let words_list = vec![String::from("train"), String::from("plane")];
        (words_list.choose(&mut rng).unwrap().to_string(), words_list)
    }

    pub fn is_valid_word(word: String, valid_words:&Vec<String>) -> bool {
        valid_words.contains(&word)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn word_in_list_valid(){
            let words = vec![String::from("train"), String::from("plane")];
            let test_word = String::from("plane");

            assert!(is_valid_word(test_word, &words));
            assert_eq!(words.iter().len(), 2);
        }

        #[test]
        fn word_not_in_list_invalid(){
            let words = vec![String::from("train"), String::from("plane")];
            let test_word = String::from("word");

            assert!(!is_valid_word(test_word, &words));
            assert_eq!(words.iter().len(), 2);
        }
    }
}