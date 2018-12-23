#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_first() {
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
    fn test_parse_csv_second() {
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

    // #[test]
    // fn test_parse_colors() {
    //     assert_eq!(2 + 2, 4);
    // }
}
