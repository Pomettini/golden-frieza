use super::*;

macro_rules! INIT_COLOR_DICTIONARY {
    ($colors:ident, $dictionary:ident) => {
        let mut $colors: Color = Default::default();
        let $dictionary = Path::new("resources/test/colors.csv");
        $colors.load_dictionary(&$dictionary);
    };
}

#[test]
fn test_load_dictionary_first() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let key: &Vec<String> = colors.dictionary.get("Black").unwrap();
    let result: &Vec<String> = &vec![
        String::from("Bold"),
        String::from("Rich"),
        String::from("Power"),
    ];

    assert_eq!(key, result);
}

#[test]
fn test_load_dictionary_second() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let key: &Vec<String> = colors.dictionary.get("White").unwrap();
    let result: &Vec<String> = &vec![
        String::from("Freshness"),
        String::from("Hope"),
        String::from("Goodness"),
    ];

    assert_eq!(key, result);
}

#[test]
fn test_process_document_colors_empty() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let document = Document::from_text(&"".to_string());
    colors.count_occurences(&document);

    let mut result: HashMap<String, usize> = HashMap::new();
    result.insert(String::from("Black"), 0);
    result.insert(String::from("White"), 0);

    assert_eq!(colors.occurrences, result);
}

#[test]
fn test_process_document_colors_first() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let document = Document::from_text(&"Freshness Something Bold Something".to_string());
    colors.count_occurences(&document);

    let mut result: HashMap<String, usize> = HashMap::new();
    result.insert(String::from("Black"), 1);
    result.insert(String::from("White"), 1);

    assert_eq!(colors.occurrences, result);
}

#[test]
fn test_process_document_colors_second() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let document = Document::from_text(&"Freshness Hope Something Bold Rich Something".to_string());
    colors.count_occurences(&document);

    let mut result: HashMap<String, usize> = HashMap::new();
    result.insert(String::from("Black"), 2);
    result.insert(String::from("White"), 2);

    assert_eq!(colors.occurrences, result);
}

#[test]
fn test_process_document_colors_case_sensitive() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let document = Document::from_text(&"freshness bold something".to_string());
    colors.count_occurences(&document);

    let mut result: HashMap<String, usize> = HashMap::new();
    result.insert(String::from("Black"), 1);
    result.insert(String::from("White"), 1);

    assert_eq!(colors.occurrences, result);
}

#[test]
fn test_process_document_colors_case_ignore_puntuaction() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let document = Document::from_text(&"freshness, freshness; bold. bold: something,".to_string());
    colors.count_occurences(&document);

    let mut result: HashMap<String, usize> = HashMap::new();
    result.insert(String::from("Black"), 2);
    result.insert(String::from("White"), 2);

    assert_eq!(colors.occurrences, result);
}

#[test]
fn test_process_document_count_words() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let document = Document::from_text(&"freshness bold something hello".to_string());
    colors.count_occurences(&document);

    assert_eq!(colors.matches, 2);
}

#[test]
fn test_process_document_calculate_percengages_first() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let document = Document::from_text(&"bold rich power freshness useless hello".to_string());
    colors.count_occurences(&document);

    let percentages = calculate_percentages(&colors.occurrences, colors.matches);

    let mut result: HashMap<String, f32> = HashMap::new();
    result.insert(String::from("Black"), 75.0);
    result.insert(String::from("White"), 25.0);

    assert_eq!(percentages, result);
}

#[test]
fn test_process_document_calculate_percengages_empty() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let document = Document::from_text(&"".to_string());
    colors.count_occurences(&document);

    let percentages = calculate_percentages(&colors.occurrences, colors.matches);

    let mut result: HashMap<String, f32> = HashMap::new();
    result.insert(String::from("Black"), 0.0);
    result.insert(String::from("White"), 0.0);

    assert_eq!(percentages, result);
}

#[test]
fn test_process_document_calculate_percengages_non_matching_words() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let document = Document::from_text(&"hello cat dog".to_string());
    colors.count_occurences(&document);

    let percentages = calculate_percentages(&colors.occurrences, colors.matches);

    let mut result: HashMap<String, f32> = HashMap::new();
    result.insert(String::from("Black"), 0.0);
    result.insert(String::from("White"), 0.0);

    assert_eq!(percentages, result);
}

#[test]
fn test_load_display_colors() {
    let display_colors =
        DisplayColors::load_dictionary(Path::new("resources/test/display_colors.csv"));

    let mut result: HashMap<String, RGB> = HashMap::new();
    result.insert(String::from("Black"), [0.0, 0.0, 0.0]);
    result.insert(String::from("White"), [255.0, 255.0, 255.0]);
    result.insert(String::from("Red"), [255.0, 0.0, 0.0]);
    result.insert(String::from("Blue"), [0.0, 0.0, 255.0]);

    assert_eq!(display_colors.dictionary, result);
}

#[test]
fn test_load_display_blend_colors_first() {
    let display_colors =
        DisplayColors::load_dictionary(Path::new("resources/test/display_colors.csv"));

    let mut input: HashMap<String, f32> = HashMap::new();
    input.insert(String::from("Black"), 50.0);
    input.insert(String::from("White"), 50.0);

    let blend = display_colors.blend_colors(input);

    let output: RGB = [127.5, 127.5, 127.5];

    assert_eq!(blend, output);
}

#[test]
fn test_load_display_blend_colors_second() {
    let display_colors =
        DisplayColors::load_dictionary(Path::new("resources/test/display_colors.csv"));

    let mut input: HashMap<String, f32> = HashMap::new();
    input.insert(String::from("Black"), 25.0);
    input.insert(String::from("White"), 25.0);

    let blend = display_colors.blend_colors(input);

    let output: RGB = [63.75, 63.75, 63.75];

    assert_eq!(blend, output);
}

#[test]
fn test_load_display_blend_colors_third() {
    let display_colors =
        DisplayColors::load_dictionary(Path::new("resources/test/display_colors.csv"));

    let mut input: HashMap<String, f32> = HashMap::new();
    input.insert(String::from("White"), 25.0);
    input.insert(String::from("Red"), 25.0);

    let blend = display_colors.blend_colors(input);

    let output: RGB = [127.5, 63.75, 63.75];

    assert_eq!(blend, output);
}

#[test]
fn test_load_display_blend_colors_fourth() {
    let display_colors =
        DisplayColors::load_dictionary(Path::new("resources/test/display_colors.csv"));

    let mut input: HashMap<String, f32> = HashMap::new();
    input.insert(String::from("White"), 25.0);
    input.insert(String::from("Red"), 25.0);
    input.insert(String::from("Blue"), 50.0);

    let blend = display_colors.blend_colors(input);

    let output: RGB = [127.5, 63.75, 191.25];

    assert_eq!(blend, output);
}

#[test]
fn test_process_file() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let document = Document::from_file(Path::new("resources/test/sample_file.txt"));
    colors.count_occurences(&document.expect("Text file cannot be parsed"));

    let mut result: HashMap<String, usize> = HashMap::new();
    result.insert(String::from("Black"), 1);
    result.insert(String::from("White"), 1);

    assert_eq!(colors.occurrences, result);
}

#[test]
fn test_process_non_text_file() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let document = Document::from_file(Path::new("resources/test/image_file.png"));

    assert_eq!(document, None);
}

#[test]
fn test_process_website() {
    INIT_COLOR_DICTIONARY!(colors, dictionary);

    let document =
        Document::from_website("https://www.giorgiopomettini.eu/test/sample_webpage.html");
    colors.count_occurences(&document);

    let mut result: HashMap<String, usize> = HashMap::new();
    result.insert(String::from("Black"), 1);
    result.insert(String::from("White"), 1);

    assert_eq!(colors.occurrences, result);
}
