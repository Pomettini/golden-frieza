extern crate csv;
extern crate golden_frieza;
extern crate iui;
// extern crate ui_sys;

use golden_frieza::*;
use iui::controls::{
    Area, AreaDrawParams, AreaHandler, Button, HorizontalBox, HorizontalSeparator, Label,
    LayoutStrategy, MultilineEntry, Spacer, VerticalBox,
};
use iui::draw::{Brush, DrawContext, FillMode, LineCap, LineJoin, SolidBrush, StrokeParams};
use iui::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::path::Path;
// use ui_sys::uiDrawContext;

/* START TODO */
// * Display color
// * Load file
// * Load website
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

    // Text labels
    let mut text_labels: HashMap<String, Label> = HashMap::new();

    // Create the input controls
    let mut input_vbox = VerticalBox::new(&ui);
    let entry = MultilineEntry::new(&ui);
    let load_file_button = Button::new(&ui, "Load File");
    let load_website_button = Button::new(&ui, "Load Website");
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
    for key in colors.dictionary.keys() {
        let mut label = Label::new(&ui, &format!("{} is 0%", key));
        text_labels.insert(key.to_string(), label.clone());
        output_vbox.append(&ui, label.clone(), LayoutStrategy::Compact);
    }

    // output_vbox.append(&ui, HorXizontalSeparator::new(&ui), LayoutStrategy::Compact);

    // Color labels
    let mut color_label = Label::new(&ui, "Color: 0 0 0");
    output_vbox.append(&ui, color_label.clone(), LayoutStrategy::Stretchy);

    // Color area
    let color_canvas = HandleCanvas {
        color: SolidBrush {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
    };

    let mut color_area = Area::new(&ui, Box::new(color_canvas));
    output_vbox.append(&ui, color_area, LayoutStrategy::Stretchy);

    process_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            let document = Document::from_text(entry.value(&ui));
            colors.count_occurences(&document);

            let percentages = calculate_percentages(&colors.occurrences, colors.matches);

            for (key, value) in percentages.clone() {
                let text = &format!("{} is {}%", key, value);
                text_labels.get_mut(&key).unwrap().set_text(&ui, text);
            }

            let color = display_colors.blend_colors(percentages);

            color_label.set_text(
                &ui,
                &format!("Color: {} {} {}", color[0], color[1], color[2]),
            );

            let color_canvas = HandleCanvas {
                color: SolidBrush {
                    r: color[0] as f64 / 255.0,
                    g: color[1] as f64 / 255.0,
                    b: color[2] as f64 / 255.0,
                    a: 1.0,
                },
            };

            let mut color_area = Area::new(&ui, Box::new(color_canvas));
            output_vbox.append(&ui, color_area, LayoutStrategy::Stretchy);
        }
    });

    // Set up the application's layout
    let mut window = Window::new(&ui, "Golden Frieza", 640, 480, WindowType::NoMenubar);
    window.set_child(&ui, horizontal_box);
    window.show(&ui);

    ui.main();
}
