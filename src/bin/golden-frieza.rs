extern crate csv;
extern crate golden_frieza;
extern crate iui;
// extern crate ui_sys;

use golden_frieza::*;
use iui::controls::{
    Area, AreaDrawParams, AreaHandler, Button, Entry, HorizontalBox, HorizontalSeparator, Label,
    LayoutStrategy, MultilineEntry, ProgressBar, ProgressBarValue, Spacer, VerticalBox,
};
use iui::draw::{Brush, FillMode, SolidBrush};
use iui::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;

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
    let colors: Rc<RefCell<Color>> = Rc::new(RefCell::new(Default::default()));
    let dictionary = Path::new("resources/colors.csv");
    colors.borrow_mut().load_dictionary(&dictionary);

    let display_colors: Rc<RefCell<DisplayColors>> = Rc::new(RefCell::new(
        DisplayColors::load_dictionary(Path::new("resources/display_colors.csv")),
    ));

    // Initialize the UI
    let ui = UI::init().unwrap();

    let mut horizontal_box = HorizontalBox::new(&ui);

    // Initialize the window
    let mut window = Window::new(&ui, "Golden Frieza", 800, 600, WindowType::NoMenubar);

    // Text labels and bars
    let text_labels: Rc<RefCell<HashMap<String, Label>>> = Rc::new(RefCell::new(HashMap::new()));
    let text_bars: Rc<RefCell<HashMap<String, ProgressBar>>> =
        Rc::new(RefCell::new(HashMap::new()));

    // Create the input controls
    let mut input_vbox = VerticalBox::new(&ui);
    let mut entry = MultilineEntry::new(&ui);
    let mut load_file_button = Button::new(&ui, "Load File");
    let mut website_entry = Entry::new(&ui);
    let mut load_website_button = Button::new(&ui, "Load Website");
    let mut clear_textarea_buton = Button::new(&ui, "Clear Text Area");
    let mut process_button = Button::new(&ui, "Process Data");

    let mut website_hbox = HorizontalBox::new(&ui);
    website_hbox.append(&ui, website_entry.clone(), LayoutStrategy::Stretchy);
    website_hbox.append(&ui, load_website_button.clone(), LayoutStrategy::Compact);
    website_hbox.set_padded(&ui, true);

    // Add to the input panel
    input_vbox.append(&ui, load_file_button.clone(), LayoutStrategy::Compact);
    input_vbox.append(&ui, website_hbox.clone(), LayoutStrategy::Compact);
    input_vbox.append(&ui, clear_textarea_buton.clone(), LayoutStrategy::Compact);
    input_vbox.append(&ui, entry.clone(), LayoutStrategy::Stretchy);
    input_vbox.append(&ui, process_button.clone(), LayoutStrategy::Compact);
    input_vbox.set_padded(&ui, true);

    horizontal_box.append(&ui, input_vbox, LayoutStrategy::Stretchy);
    horizontal_box.set_padded(&ui, true);

    let mut output_vbox = VerticalBox::new(&ui);
    horizontal_box.append(&ui, output_vbox.clone(), LayoutStrategy::Stretchy);

    // Insert GUI elements in a dictionary where Color is the key
    for key in colors.borrow().dictionary.keys() {
        let mut label = Label::new(&ui, &format!("{} is 0%", key));
        text_labels
            .borrow_mut()
            .insert(key.to_string(), label.clone());
        output_vbox.append(&ui, label.clone(), LayoutStrategy::Compact);

        let mut text_bar = ProgressBar::new();
        text_bars
            .borrow_mut()
            .insert(key.to_string(), text_bar.clone());
        output_vbox.append(&ui, text_bar.clone(), LayoutStrategy::Compact);
    }

    // Color labels
    let mut color_label = Label::new(&ui, "Color: 0 0 0");
    output_vbox.append(&ui, color_label.clone(), LayoutStrategy::Stretchy);

    output_vbox.append(&ui, HorizontalSeparator::new(&ui), LayoutStrategy::Stretchy);

    // Color area
    let color_canvas = HandleCanvas {
        color: SolidBrush {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    };

    // This is a way to update a color area
    // It's a terrible, terrible hack, but it works
    // And it will leak memory :(

    let mut temp_vboxes = Rc::new(RefCell::new(Vec::new()));

    let mut temp_vbox = VerticalBox::new(&ui);
    temp_vboxes.borrow_mut().push(temp_vbox.clone());

    let color_area = Area::new(&ui, Box::new(color_canvas));
    temp_vbox.append(&ui, color_area, LayoutStrategy::Compact);
    output_vbox.append(&ui, temp_vbox.clone(), LayoutStrategy::Compact);

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
        let entry = entry.clone();
        let colors = colors.clone();
        let text_labels = text_labels.clone();
        let text_bars = text_bars.clone();
        let display_colors = display_colors.clone();
        let color_label = color_label.clone();
        let temp_vboxes = temp_vboxes.clone();
        let output_vbox = output_vbox.clone();
        move |_| {
            let document = Document::from_text(&entry.value(&ui));
            colors.borrow_mut().count_occurences(&document);

            refresh_content!(
                ui,
                colors,
                text_labels,
                text_bars,
                display_colors,
                color_label,
                temp_vboxes,
                output_vbox
            );
        }
    });

    load_file_button.on_clicked(&ui, {
        let ui = ui.clone();
        let entry = entry.clone();
        let colors = colors.clone();
        move |_| {
            let path = window.open_file(&ui).expect("File not found");
            let document = Document::from_file(&path);
            colors.borrow_mut().count_occurences(&document);

            refresh_content!(
                ui,
                colors,
                text_labels,
                text_bars,
                display_colors,
                color_label,
                temp_vboxes,
                output_vbox
            );
        }
    });

    load_website_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            let document = Document::from_website("http://example.org/");
        }
    });

    ui.main();
}

#[macro_export]
macro_rules! refresh_content {
    ($ui:expr, $colors:expr, $text_labels:expr, $text_bars:expr, $display_colors:expr, $color_label:expr, $temp_vboxes:expr, $output_vbox:expr) => {
        let percentages =
            calculate_percentages(&$colors.borrow().occurrences, $colors.borrow().matches);

        for (key, value) in percentages.clone() {
            let text = &format!("{} is {:.0}%", key, value);
            $text_labels
                .borrow_mut()
                .get_mut(&key)
                .unwrap()
                .set_text(&$ui, text);
            $text_bars
                .borrow_mut()
                .get_mut(&key)
                .unwrap()
                .set_value(ProgressBarValue::Determinate(value as u32));
        }

        let color = $display_colors.borrow().blend_colors(percentages);

        $color_label.clone().set_text(
            &$ui,
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

        let mut t = $temp_vboxes.borrow_mut().pop().unwrap();
        t.hide(&$ui);

        let mut temp_vbox = VerticalBox::new(&$ui);
        $temp_vboxes.borrow_mut().push(temp_vbox.clone());

        let color_area = Area::new(&$ui, Box::new(color_canvas));
        temp_vbox.append(&$ui, color_area, LayoutStrategy::Compact);
        $output_vbox.append(&$ui, temp_vbox.clone(), LayoutStrategy::Compact);
    };
}
