extern crate csv;
extern crate golden_frieza;
extern crate iui;

use golden_frieza::*;
use iui::controls::{Button, MultilineEntry, VerticalBox};
use iui::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut colors: Color = Default::default();
    let dictionary = Path::new("resources/test/colors.csv");
    colors.load_dictionary(&dictionary);

    for key in colors.dictionary.keys() {
        println!("{}", key);
    }

    // Initialize the UI
    let ui = UI::init().unwrap();

    // Set up the application's layout
    let mut window = Window::new(&ui, "Golden Frieza", 640, 480, WindowType::NoMenubar);
    window.show(&ui);

    ui.main();
}
