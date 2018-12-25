extern crate csv;
extern crate golden_frieza;
extern crate iui;
extern crate ui_sys;

use golden_frieza::*;
use iui::controls::{
    Button, HorizontalBox, HorizontalSeparator, Label, MultilineEntry, Spacer, VerticalBox,
};
use iui::draw::{Brush, DrawContext, FillMode, LineCap, LineJoin, SolidBrush, StrokeParams};
use iui::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use ui_sys::uiDrawContext;

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

    // output_vbox.append(&ui, HorizontalSeparator::new(&ui), LayoutStrategy::Compact);

    // Color labels
    let mut color_label = Label::new(&ui, "Color: 0 0 0");
    output_vbox.append(&ui, color_label.clone(), LayoutStrategy::Stretchy);

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
        }
    });

    // unsafe {
    //     let mut draw_ctx: uiDrawContext;
    //     let draw_context = iui::draw::DrawContext::from_ui_draw_context(&mut draw_ctx);
    //     let stroke = StrokeParams {
    //         cap: LineCap::Flat,
    //         join: LineJoin::Miter,
    //         thickness: 1.0,
    //         miter_limit: 1.0,
    //         dashes: Vec::new(),
    //         dash_phase: 1.0,
    //     };
    //     let brush = SolidBrush {
    //         r: 255.0,
    //         g: 0.0,
    //         b: 0.0,
    //         a: 0.0,
    //     };

    //     let draw_path = iui::draw::Path::new(&ui, FillMode::Winding);
    //     draw_context.stroke(&ui, &draw_path, &Brush::Solid(brush), &stroke);
    // }

    // Set up the application's layout
    let mut window = Window::new(&ui, "Golden Frieza", 640, 480, WindowType::NoMenubar);
    window.set_child(&ui, horizontal_box);
    window.show(&ui);

    ui.main();
}
