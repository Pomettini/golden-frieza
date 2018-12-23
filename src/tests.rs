use super::*;

#[test]
fn test_load_dictionary_first() {
    let mut colors: Color = Default::default();
    let dictionary = Path::new("resources/test/colors.csv");
    colors.load_dictionary(&dictionary);

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
    let mut colors: Color = Default::default();
    let dictionary = Path::new("resources/test/colors.csv");
    colors.load_dictionary(&dictionary);

    let key: &Vec<String> = colors.dictionary.get("White").unwrap();
    let result: &Vec<String> = &vec![
        String::from("Freshness"),
        String::from("Hope"),
        String::from("Goodness"),
    ];

    assert_eq!(key, result);
}

#[test]
fn test_process_document_colors_first() {
    let mut colors: Color = Default::default();
    let dictionary = Path::new("resources/test/colors.csv");
    colors.load_dictionary(&dictionary);

    let document = Document::from_text("Freshness Something Bold Something".to_string());
    colors.count_occurences(&document);

    let mut result: HashMap<String, usize> = HashMap::new();
    result.insert(String::from("Black"), 2);
    result.insert(String::from("White"), 2);

    assert_eq!(colors.occurrences, result);
}
