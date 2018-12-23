extern crate csv;

use csv::{Reader, ReaderBuilder};
use std::collections::HashMap;
use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;

pub trait Element {
    fn load_dictionary(&mut self, path: &Path);
    fn count_occurences(&mut self, document: &Document);
}

#[derive(Default)]
pub struct Color {
    pub dictionary: HashMap<String, Vec<String>>,
    pub occurrences: HashMap<String, usize>,
    pub matches: usize,
}

#[derive(Default)]
pub struct Document {
    pub content: String,
}

impl Document {
    pub fn from_text(text: String) -> Document {
        Document { content: text }
    }

    pub fn from_path(path: &Path) {
        unimplemented!();
    }

    pub fn from_website() {
        unimplemented!();
    }
}

impl Element for Color {
    fn load_dictionary(&mut self, path: &Path) {
        let mut dictionary = ReaderBuilder::new()
            .delimiter(b';')
            .from_path(path)
            .unwrap();

        for result in dictionary.deserialize() {
            let record: (String, String) = result.unwrap();

            let color = record.0;
            let words: Vec<String> = Vec::from_iter(record.1.split(", ").map(String::from));

            self.dictionary.insert(color, words);
        }
    }

    fn count_occurences(&mut self, document: &Document) {
        let words: Vec<String> = Vec::from_iter(document.content.split(" ").map(String::from));

        for key in self.dictionary.keys() {
            let mut counter: usize = 0;

            for word in &words {
                let values = self.dictionary.get(key).unwrap();

                for value in values {
                    // TODO: Fix it, extremely inefficent
                    if value.to_lowercase() == word.to_lowercase() {
                        counter += 1;
                        self.matches += 1;
                    }
                }
            }

            self.occurrences.insert(key.to_string(), counter);
        }
    }
}

pub fn calculate_percentages(
    occurences: &HashMap<String, usize>,
    matches: usize,
) -> HashMap<String, f32> {
    let mut result: HashMap<String, f32> = HashMap::new();

    for (key, value) in occurences {
        let percentage = (*value as f32 / matches as f32) * 100.0;
        result.insert(key.to_string(), percentage);
    }

    result
}

#[cfg(test)]
mod tests;
