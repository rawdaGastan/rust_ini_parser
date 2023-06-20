mod parser;

pub use parser::ini::*;

fn main() {
    let mut parser: Parser = Parser::new();
    parser
        .from_file(&String::from("src/test.ini"))
        .expect("Should read from file");
}
