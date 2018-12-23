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

        println!("{:?}", dictionary);

        for result in dictionary.deserialize() {
            let record: (String, String) = result.unwrap();

            println!("{:?}", &record);

            let color = record.0;
            let words: Vec<String> = Vec::from_iter(record.1.split(", ").map(String::from));

            self.dictionary.insert(color, words);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_first() {
        let mut colors: Color = Default::default();
        let dictionary = Path::new("resources/test/colors.csv");
        colors.load_dictionary(&dictionary);

        let key: &Vec<String> = colors.dictionary.get("Black").unwrap();
        let result: &Vec<String> = &vec![
            String::from("Bold"),
            String::from("Rich"),
            String::from("Power"),
        ];

        assert_eq!(key, result);
    }

    #[test]
    fn test_parse_csv_second() {
        let mut colors: Color = Default::default();
        let dictionary = Path::new("resources/test/colors.csv");
        colors.load_dictionary(&dictionary);

        let key: &Vec<String> = colors.dictionary.get("White").unwrap();
        let result: &Vec<String> = &vec![
            String::from("Freshness"),
            String::from("Hope"),
            String::from("Goodness"),
        ];

        assert_eq!(key, result);
    }

    // #[test]
    // fn test_parse_colors() {
    //     assert_eq!(2 + 2, 4);
    // }
}
