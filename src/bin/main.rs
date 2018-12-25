extern crate csv;
extern crate golden_frieza;
extern crate iui;

use golden_frieza::*;
use iui::controls::{Button, HorizontalBox, Label, MultilineEntry, VerticalBox};
use iui::prelude::*;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let mut colors: Color = Default::default();
    let dictionary = Path::new("resources/colors.csv");
    colors.load_dictionary(&dictionary);

    // Initialize the UI
    let ui = UI::init().unwrap();

    let mut horizontal_box = HorizontalBox::new(&ui);

    // Color text labels
    let mut text_labels: HashMap<String, Label> = HashMap::new();

    // Create the input controls
    let mut input_vbox = VerticalBox::new(&ui);
    let mut entry = MultilineEntry::new(&ui);
    let mut load_file_button = Button::new(&ui, "Load File");
    let mut load_website_button = Button::new(&ui, "Load Website");
    let mut process_button = Button::new(&ui, "Process Data");

    input_vbox.append(&ui, load_file_button.clone(), LayoutStrategy::Compact);
    input_vbox.append(&ui, load_website_button.clone(), LayoutStrategy::Compact);
    input_vbox.append(&ui, entry.clone(), LayoutStrategy::Stretchy);
    input_vbox.append(&ui, process_button.clone(), LayoutStrategy::Compact);
    input_vbox.set_padded(&ui, true);

    horizontal_box.append(&ui, input_vbox, LayoutStrategy::Stretchy);
    horizontal_box.set_padded(&ui, true);

    let mut output_vbox = VerticalBox::new(&ui);
    horizontal_box.append(&ui, output_vbox.clone(), LayoutStrategy::Stretchy);

    // Insert GUI elements in a dictionary where Color is the key
    for (key, _) in &colors.dictionary {
        let mut label = Label::new(&ui, &format!("{}", key));
        text_labels.insert(key.to_string(), label.clone());
        output_vbox.append(&ui, label.clone(), LayoutStrategy::Compact);
    }

    process_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            let document = Document::from_text(entry.value(&ui));
            colors.count_occurences(&document);

            let percentages = calculate_percentages(&colors.occurrences, colors.matches);

            for (key, value) in percentages {
                let text = &format!("{} is {}%", key, value);
                text_labels.get_mut(&key).unwrap().set_text(&ui, text);
            }
        }
    });

    // Set up the application's layout
    let mut window = Window::new(&ui, "Golden Frieza", 640, 480, WindowType::NoMenubar);
    window.set_child(&ui, horizontal_box);
    window.show(&ui);

    ui.main();
}
