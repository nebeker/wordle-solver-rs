pub mod word_picker {
    use rand::prelude::IndexedRandom;
    use crate::dictionay_parser::dictionary_parser;

    pub fn pick_word() ->String{
        let words = dictionary_parser::parse_file_for_words("en-US.dic");

        let mut rng = rand::rng();

        words.choose(&mut rng).unwrap().to_string()
    }
}