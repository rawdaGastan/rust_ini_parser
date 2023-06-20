#[cfg(test)]
mod ini_tests {
    use super::super::ini::*;
    use indexmap::IndexMap;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref CONTENT: IndexMap<String, String> = {
            let mut m = IndexMap::new();

            //valid options
            m.insert(String::from("valid"), String::from("[owner]\nname=John\norganization = threefold\n\n[database]\nserver = 192.0.2.62\nport = 143\npassword = 123456\nprotected = true\nversion = 12.6"));
            m.insert(String::from("valid_comment"), String::from(";comment"));
            m.insert(String::from("valid_empty"), String::from(""));

            //invalid options
            m.insert(String::from("invalid"), String::from("[owner]\n--"));
            m.insert(String::from("invalid_section"), String::from("[[owner]"));
            m.insert(String::from("invalid_unclosed_section"), String::from("[owner\nkey=val\n"));
            m.insert(String::from("invalid_unopened_section"), String::from("owner]\nkey=val\n"));
            m.insert(String::from("invalid_no_equal"), String::from("[owner]\nkey_value"));
            m.insert(String::from("invalid_no_value"), String::from("[owner]\nkey_value="));
            m.insert(String::from("invalid_no_key"), String::from("[owner]\n=nkey_value"));
            m.insert(String::from("invalid_more_than_one_equal"), String::from("[owner]\nkey==val"));
            m.insert(String::from("invalid_no_sections"), String::from(""));
            m.insert(String::from("invalid_no_options"), String::from("[owner]"));
            m
        };
    }

    #[test]
    fn valid_test() {
        let mut parser: Parser = Parser::new();
        let content = CONTENT.get("valid").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());
    }

    #[test]
    fn valid_comment_test() {
        let mut parser: Parser = Parser::new();
        let content = CONTENT.get("valid_comment").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());
    }

    #[test]
    fn valid_empty_test() {
        let mut parser: Parser = Parser::new();
        let content = CONTENT.get("valid_empty").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());
    }

    #[test]
    fn values_test() {
        let mut parser: Parser = Parser::new();

        let content = CONTENT.get("valid").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        let option = parser.get_option("owner", "name").unwrap();
        assert_eq!(option, "John");

        let option = parser.get_option("owner", "organization").unwrap();
        assert_eq!(option, "threefold");

        let option = parser.get_option("database", "server").unwrap();
        assert_eq!(option, "192.0.2.62");

        let option = parser.get_option("database", "port").unwrap();
        assert_eq!(option, "143");

        let option = parser.get_option("database", "password").unwrap();
        assert_eq!(option, "123456");

        let option = parser.get_option("database", "protected").unwrap();
        assert_eq!(option, "true");

        let option = parser.get_option("database", "version").unwrap();
        assert_eq!(option, "12.6");

        let option = parser.get_bool("database", "protected").unwrap();
        assert!(option);

        let option = parser.get_int("database", "port").unwrap();
        assert_eq!(option, 143);

        let option = parser.get_float("database", "port").unwrap();
        assert_eq!(option, 143.0);

        let option = parser.get_int("database", "password").unwrap();
        assert_eq!(option, 123456);

        let option = parser.get_float("database", "version").unwrap();
        assert_eq!(option, 12.6);
    }

    #[test]
    fn parsed_sections_test() {
        let mut parser: Parser = Parser::new();

        let content = CONTENT.get("valid").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        let got = parser.get_sections();
        let want = ["owner", "database"];
        assert_eq!(got, want);
    }

    #[test]
    fn parsed_section_test() {
        let mut parser: Parser = Parser::new();

        let content = CONTENT.get("valid").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        let got = parser.get_section("owner").unwrap();
        let mut want = IndexMap::new();
        want.insert(String::from("name"), String::from("John"));
        want.insert(String::from("organization"), String::from("threefold"));
        assert_eq!(got, want);
    }

    #[test]
    fn parsed_options_test() {
        let mut parser: Parser = Parser::new();

        let content = CONTENT.get("valid").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        let got = parser.get_options("owner").unwrap();
        let want = ["name", "organization"];
        assert_eq!(got, want);
    }

    #[test]
    fn set_option_test() {
        let mut parser: Parser = Parser::new();

        let content = CONTENT.get("valid").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        assert!(!parser.set_option("owner", "name", "Rawda").is_err());

        let option = parser.get_option("owner", "name").unwrap();
        assert_eq!(option, "Rawda");
    }

    #[test]
    fn parsed_map_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("valid").unwrap().to_string();

        assert!(!parser.from_string(content).is_err());
        let parsed = parser.parsed_map();

        let test_parsed_str = parser.string();
        assert!(!parser.from_string(test_parsed_str).is_err());
        let test_parsed_dict = parser.parsed_map();

        assert_eq!(test_parsed_dict, parsed);
    }

    #[test]
    fn set_option_no_option_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("valid").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        assert!(!parser.set_option("owner", "age", "30").is_err());

        let option = parser.get_option("owner", "age").unwrap();
        assert_eq!(option, "30");
    }

    #[test]
    fn no_sections_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("invalid_no_sections").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        let got = parser.get_sections();
        assert_eq!(got.len(), 0);
    }

    #[test]
    fn no_options_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("invalid_no_options").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        let got = parser.get_options("owner").unwrap();
        assert_eq!(got.len(), 0);
    }

    #[test]
    fn invalid_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("invalid").unwrap().to_string();
        assert!(parser.from_string(content).is_err());
    }

    #[test]
    fn invalid_section_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("invalid_section").unwrap().to_string();
        assert!(parser.from_string(content).is_err());
    }

    #[test]
    fn invalid_unclosed_section_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("invalid_unclosed_section").unwrap().to_string();
        assert!(parser.from_string(content).is_err());
    }

    #[test]
    fn invalid_unopened_section_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("invalid_unopened_section").unwrap().to_string();
        assert!(parser.from_string(content).is_err());
    }

    #[test]
    fn invalid_no_equal_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("invalid_no_equal").unwrap().to_string();
        assert!(parser.from_string(content).is_err());
    }

    #[test]
    fn invalid_no_value_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("invalid_no_value").unwrap().to_string();
        assert!(parser.from_string(content).is_err());
    }

    #[test]
    fn invalid_no_key_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("invalid_no_key").unwrap().to_string();
        assert!(parser.from_string(content).is_err());
    }

    #[test]
    fn invalid_more_than_one_equal_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT
            .get("invalid_more_than_one_equal")
            .unwrap()
            .to_string();
        assert!(parser.from_string(content).is_err());
    }

    #[test]
    fn wrong_section_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("valid").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        assert!(parser.get_section("owners").is_err());
    }

    #[test]
    fn wrong_option_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("valid").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        assert!(parser.get_option("owner", "server").is_err());
    }

    #[test]
    fn wrong_bool_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("valid").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        assert!(parser.get_bool("owner", "server").is_err());
    }

    #[test]
    fn wrong_int_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("valid").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        assert!(parser.get_int("owner", "server").is_err());
    }

    #[test]
    fn wrong_float_test() {
        let mut parser: Parser = Parser::new();

        let content: String = CONTENT.get("valid").unwrap().to_string();
        assert!(!parser.from_string(content).is_err());

        assert!(parser.get_float("owner", "server").is_err());
    }
}
