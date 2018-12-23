extern crate csv;
extern crate golden_frieza;

use golden_frieza::*;
use std::path::Path;

fn main() {
    let mut colors: Color = Default::default();
    let dictionary = Path::new("resources/test/colors.csv");
    colors.load_dictionary(&dictionary);

    for key in colors.dictionary.keys() {
        println!("{}", key);
    }
}
