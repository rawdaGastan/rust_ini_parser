mod parser;

use std::env;

pub use parser::ini::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file_path = &args[1];

        let mut parser: Parser = Parser::new();

        parser
            .from_file(String::from("src/test.ini"))
            .expect("Should read from file");
        parser
            .save_to_file(file_path.to_string())
            .expect("Should write to file");
    }
}
