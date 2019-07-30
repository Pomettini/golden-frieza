#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

extern crate ammonia;
extern crate csv;
extern crate regex;
extern crate reqwest;

use ammonia::Builder;
use csv::Reader;
use csv::ReaderBuilder;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path::Path;

type RGB = [f32; 3];

#[derive(Default)]
pub struct DisplayColors {
    pub dictionary: HashMap<String, RGB>,
}

impl DisplayColors {
    pub fn load_dictionary(path: &Path) -> Result<Self, &str> {
        let mut reader = match Reader::from_path(path) {
            Ok(reader_) => reader_,
            Err(_) => return Err("Please enter a valid color dictionary file"),
        };
        let mut dictionary: HashMap<String, RGB> = HashMap::new();

        for record in reader.deserialize() {
            // TODO: Refactor, there has to be a better way to write this
            let record: (String, f32, f32, f32) = match record {
                Ok(record_) => record_,
                Err(_) => return Err("The dictionary file is not valid"),
            };
            let mut rgb: RGB = [0.0; 3];
            rgb[0] = record.1;
            rgb[1] = record.2;
            rgb[2] = record.3;
            dictionary.insert(record.0, rgb);
        }

        Ok(Self { dictionary })
    }

    pub fn blend_colors(&self, dictionary: HashMap<String, f32>) -> RGB {
        let mut rgb: RGB = [0.0; 3];

        for (key, value) in dictionary {
            for i in 0..3 {
                rgb[i] += (self.dictionary[&key][i] * value) / 100.0;
            }
        }

        rgb
    }
}

pub trait Element {
    fn load_dictionary(&mut self, path: &Path) -> Result<(), &str>;
    fn count_occurences(&mut self, document: &Document);
}

#[derive(Default)]
pub struct Color {
    pub dictionary: HashMap<String, Vec<String>>,
    pub occurrences: HashMap<String, usize>,
    pub matches: usize,
}

#[derive(Default, Debug, PartialEq)]
pub struct Document {
    pub content: String,
}

impl Document {
    pub fn from_text(text: &str) -> Self {
        Self {
            content: text.to_string(),
        }
    }

    pub fn from_file(path: &Path) -> Result<Self, &str> {
        let mut file = match File::open(&path) {
            Ok(file_) => file_,
            Err(_) => return Err("Cannot open the file"),
        };
        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(_) => Ok(Self { content: contents }),
            Err(_) => Err("Cannot parse the file"),
        }
    }

    pub fn from_website(url: &str) -> Option<Self> {
        // TODO: Handle errors and exceptions

        let mut request = reqwest::get(url).expect("URL not valid");
        let page_content = request.text().expect("Cannot parse page content");

        #[rustfmt::skip]
        let tags: HashSet<_> = [
            "a", "abbr", "acronym", "area", "article", "aside", "b", "bdi",
            "bdo", "blockquote", "br", "caption", "center", "cite", "code",
            "col", "colgroup", "data", "dd", "del", "details", "dfn", "div",
            "dl", "dt", "em", "figcaption", "figure", "footer", "h1", "h2",
            "h3", "h4", "h5", "h6", "header", "hgroup", "hr", "i", "img",
            "ins", "kbd", "kbd", "li", "map", "mark", "nav", "ol", "p", "pre",
            "q", "rp", "rt", "rtc", "ruby", "s", "samp", "small", "span",
            "strike", "strong", "sub", "summary", "sup", "table", "tbody",
            "td", "th", "thead", "time", "tr", "tt", "u", "ul", "var", "wbr"
        ].iter().collect();

        let clean_text = Builder::new()
            .rm_tags(tags)
            .clean(&page_content)
            .to_string();

        // Remove newlines ecc
        let re = Regex::new(r"\n|\r|\t").expect("Invalid regex");
        let clean_text = re.replace_all(&clean_text, "").to_string();

        println!("RESULT: {:?}", &clean_text);

        Some(Self {
            content: clean_text,
        })
    }
}

impl Element for Color {
    fn load_dictionary(&mut self, path: &Path) -> Result<(), &str> {
        let mut dictionary = match ReaderBuilder::new().delimiter(b';').from_path(path) {
            Ok(file) => file,
            Err(_) => return Err("Cannot read the color file"),
        };

        for result in dictionary.deserialize() {
            let record: (String, String) = match result {
                Ok(record_) => record_,
                Err(_) => return Err("Cannot deserialize the dictionary"),
            };

            let color = record.0;
            let words: Vec<String> = Vec::from_iter(record.1.split(", ").map(String::from));

            self.dictionary.insert(color, words);
        }

        Ok(())
    }

    fn count_occurences(&mut self, document: &Document) {
        // Make a dictionary of all the words of the document
        let words: Vec<String> = Vec::from_iter(
            document
                .content
                .split([' ', ',', ';', '.', ':'].as_ref())
                .map(String::from),
        );

        // Reset the matches counter
        self.matches = 0;

        for key in self.dictionary.keys() {
            let mut counter: usize = 0;

            for word in &words {
                let values = &self.dictionary[key];

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
        // If result is NAN, will return 0
        let percentage = f32::max(0.0, (*value as f32 / matches as f32) * 100.0);
        result.insert(key.to_string(), percentage);
    }

    result
}

#[cfg(test)]
mod tests;
