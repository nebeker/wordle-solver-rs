pub mod word_picker {
    use rand::prelude::IndexedRandom;
    use rand::rng;

    pub fn pick_word() ->String{
        let words = vec!["train", "plane", "river", "rusty", "rover", "arise"];

        let mut rng = rand::rng();

        words.choose(&mut rng).unwrap().to_string()
    }
}