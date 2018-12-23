extern crate csv;
extern crate golden_frieza;

use csv::Reader;
use golden_frieza::*;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut colors: Color = Default::default();
    let dictionary = Path::new("resources/test/colors.csv");
    colors.load_dictionary(&dictionary);

    for key in colors.dictionary.keys() {
        println!("{}", key);
    }
}
