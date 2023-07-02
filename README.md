# rust_ini_parser

[![Codacy Badge](https://app.codacy.com/project/badge/Grade/cd6e18aac6be404ab89ec160b4b36671)](https://app.codacy.com/gh/rawdaGastan/rust_ini_parser/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=threefoldtech/rust_ini_parser&amp;utm_campaign=Badge_Grade) [![Dependabot](https://badgen.net/badge/Dependabot/enabled/green?icon=dependabot)](https://dependabot.com/) [![Testing](https://github.com/rawdagastan/rust_ini_parser/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rawdagastan/rust_ini_parser/actions/workflows/rust.yml) <a href='https://github.com/jpoles1/gopherbadger' target='_blank'>![gopherbadger-tag-do-not-edit](https://img.shields.io/badge/Coverage-96.23%25-brightgreen.svg?longCache=true&style=flat)</a>

## How to use

- Create a new *.ini file and insert your ini content, for example:
  
```ini
[owner]
name = John
organization = threefold

[database]
version = 12.6
server = 192.0.2.62
port = 143
password = 123456
protected = true
```

- Create a new parser

```rust
mod parser;
pub use parser::ini::*;

let mut parser: Parser = Parser::new();
```

- You can parse a file

```rust
parser.from_file(String::from("INI_FILE_PATH")).expect("Should read from file");
```

## Functions

- `parser.parsed_map()` &rarr; to get your parsed map
- `parser.from_string( content )` &rarr; to convert your ini string to a parsed map
- `parser.to_string()` &rarr; to convert your parsed map to ini string
- `parser.from_file( file_path )` &rarr; to convert your ini file to a parsed map
- `parser.save_to_file( file_path )` &rarr; to save your ini string converted parsed map
- `parser.get_sections()` &rarr; to get your sections' names
- `parser.get_section( section_key )` &rarr; to get the content of the specified section key
- `parser.get_options( section_key )` &rarr; to get the options of the specified section key
- `parser.get_option( section_key, option_key )` &rarr; to get the string value of an option key inside a section
- `parser.set_option( section_key, option_key )` &rarr; to set the string value of an option inside a section
- `parser.get_bool( section_key, option_key )` &rarr; to set the bool value of an option inside a section
- `parser.get_int( section_key, option_key )` &rarr; to set the integer value of an option inside a section
- `parser.get_float( section_key, option_key )` &rarr; to set the float value of an option inside a section

## Build

```bash
cargo build
```

```bash
make build
```

## Testing

Use those commands to run the tests

```bash
cargo test
```

```bash
make test
```

## Coverage

```bash
make coverage
```
