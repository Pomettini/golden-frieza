extern crate csv;
extern crate golden_frieza;
extern crate iui;
// extern crate ui_sys;

use golden_frieza::*;
use iui::controls::{
    Area, AreaDrawParams, AreaHandler, Button, Entry, HorizontalBox, Label, LayoutStrategy,
    MultilineEntry, VerticalBox,
};
use iui::draw::{Brush, FillMode, SolidBrush};
use iui::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/* START TODO */
// * Load file
// * Load website
// * Support multiple languages
/* END TODO */

struct HandleCanvas {
    color: SolidBrush,
}

impl AreaHandler for HandleCanvas {
    fn draw(&mut self, _area: &Area, draw_params: &AreaDrawParams) {
        let ctx = &draw_params.context;

        let path = iui::draw::Path::new(ctx, FillMode::Winding);
        path.add_rectangle(ctx, 0., 0., draw_params.area_width, draw_params.area_height);
        path.end(ctx);

        let brush = Brush::Solid(self.color);

        draw_params.context.fill(&path, &brush);
    }
}

fn main() {
    let mut colors: Color = Default::default();
    let dictionary = Path::new("resources/colors.csv");
    colors.load_dictionary(&dictionary);

    let display_colors: DisplayColors =
        DisplayColors::load_dictionary(Path::new("resources/display_colors.csv"));

    // Initialize the UI
    let ui = UI::init().unwrap();

    let mut horizontal_box = HorizontalBox::new(&ui);

    // Initialize the window
    let mut window = Window::new(&ui, "Golden Frieza", 640, 480, WindowType::NoMenubar);

    // Text labels
    let mut text_labels: HashMap<String, Label> = HashMap::new();

    // Create the input controls
    let mut input_vbox = VerticalBox::new(&ui);
    let mut entry = MultilineEntry::new(&ui);
    let mut load_file_button = Button::new(&ui, "Load File");
    let mut load_website_button = Button::new(&ui, "Load Website");
    let mut clear_textarea_buton = Button::new(&ui, "Clear Text Area");
    let mut process_button = Button::new(&ui, "Process Data");

    input_vbox.append(&ui, load_file_button.clone(), LayoutStrategy::Compact);
    input_vbox.append(&ui, load_website_button.clone(), LayoutStrategy::Compact);
    input_vbox.append(&ui, clear_textarea_buton.clone(), LayoutStrategy::Compact);
    input_vbox.append(&ui, entry.clone(), LayoutStrategy::Stretchy);
    input_vbox.append(&ui, process_button.clone(), LayoutStrategy::Compact);
    input_vbox.set_padded(&ui, true);

    horizontal_box.append(&ui, input_vbox, LayoutStrategy::Stretchy);
    horizontal_box.set_padded(&ui, true);

    let mut output_vbox = VerticalBox::new(&ui);
    horizontal_box.append(&ui, output_vbox.clone(), LayoutStrategy::Stretchy);

    // Insert GUI elements in a dictionary where Color is the key
    for key in colors.dictionary.keys() {
        let mut label = Label::new(&ui, &format!("{} is 0%", key));
        text_labels.insert(key.to_string(), label.clone());
        output_vbox.append(&ui, label.clone(), LayoutStrategy::Compact);
    }

    // Color labels
    let mut color_label = Label::new(&ui, "Color: 0 0 0");
    output_vbox.append(&ui, color_label.clone(), LayoutStrategy::Stretchy);

    // Color area
    let color_canvas = HandleCanvas {
        color: SolidBrush {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        },
    };

    // This is a way to update a color area
    // It's a terrible, terrible hack, but it works
    // And it will leak memory :(

    let mut temp_vboxes = Vec::new();

    let mut temp_vbox = VerticalBox::new(&ui);
    temp_vboxes.push(temp_vbox.clone());

    let color_area = Area::new(&ui, Box::new(color_canvas));
    temp_vbox.append(&ui, color_area, LayoutStrategy::Stretchy);
    output_vbox.append(&ui, temp_vbox.clone(), LayoutStrategy::Stretchy);

    window.set_child(&ui, horizontal_box);
    window.show(&ui);

    clear_textarea_buton.on_clicked(&ui, {
        let ui = ui.clone();
        let mut entry = entry.clone();
        move |_| {
            entry.set_value(&ui, "");
        }
    });

    process_button.on_clicked(&ui, {
        let ui = ui.clone();
        let mut entry = entry.clone();
        move |_| {
            let document = Document::from_text(&entry.value(&ui));
            colors.count_occurences(&document);

            let percentages = calculate_percentages(&colors.occurrences, colors.matches);

            for (key, value) in percentages.clone() {
                let text = &format!("{} is {:.0}%", key, value);
                text_labels.get_mut(&key).unwrap().set_text(&ui, text);
            }

            let color = display_colors.blend_colors(percentages);

            color_label.set_text(
                &ui,
                &format!("Color: {:.0} {:.0} {:.0}", color[0], color[1], color[2]),
            );

            let color_canvas = HandleCanvas {
                color: SolidBrush {
                    r: f64::from(color[0]) / 255.0,
                    g: f64::from(color[1]) / 255.0,
                    b: f64::from(color[2]) / 255.0,
                    a: 1.0,
                },
            };

            // TODO: Remove this terrible hack

            let mut t = temp_vboxes.pop().unwrap();
            t.hide(&ui);

            let mut temp_vbox = VerticalBox::new(&ui);
            temp_vboxes.push(temp_vbox.clone());

            let color_area = Area::new(&ui, Box::new(color_canvas));
            temp_vbox.append(&ui, color_area, LayoutStrategy::Stretchy);
            output_vbox.append(&ui, temp_vbox.clone(), LayoutStrategy::Stretchy);
        }
    });

    load_file_button.on_clicked(&ui, {
        let ui = ui.clone();
        let mut entry = entry.clone();
        move |_| {
            let path = window.open_file(&ui);
            let mut file = File::open(&path.unwrap()).expect("File not found");
            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("Cannot read the file");

            entry.set_value(&ui, &contents);
        }
    });

    load_website_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            let mut website_window =
                Window::new(&ui, "Load website", 640, 60, WindowType::NoMenubar);
            let mut website_entry = Entry::new(&ui);
            let mut website_save_button = Button::new(&ui, "Save Buffer");
            let mut website_vbox = VerticalBox::new(&ui);
            website_vbox.append(&ui, website_entry.clone(), LayoutStrategy::Compact);
            website_vbox.append(&ui, website_save_button.clone(), LayoutStrategy::Compact);
            website_window.set_child(&ui, website_vbox);
            website_window.show(&ui);
        }
    });

    ui.main();
}
