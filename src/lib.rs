extern crate csv;

use csv::{Reader, ReaderBuilder};
use std::collections::HashMap;
use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;

pub trait Element {
    fn load_dictionary(&mut self, path: &Path);
}

#[derive(Default)]
pub struct Color {
    pub dictionary: HashMap<String, Vec<String>>,
}

impl Element for Color {
    fn load_dictionary(&mut self, path: &Path) {
        let mut dictionary = ReaderBuilder::new()
            .delimiter(b';')
            .from_path(path)
            .unwrap();

        for result in dictionary.deserialize() {
            let record: (String, String) = result.unwrap();

            println!("{:?}", &record);

            let color = record.0;
            let words: Vec<String> = Vec::from_iter(record.1.split(", ").map(String::from));

            self.dictionary.insert(color, words);
        }
    }
}
