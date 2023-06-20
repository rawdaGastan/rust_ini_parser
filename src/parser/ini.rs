use indexmap::IndexMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};

#[derive(Debug, Default)]
pub struct Parser {
    parsed_map: IndexMap<String, IndexMap<String, String>>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            parsed_map: IndexMap::new(),
        }
    }

    pub fn parsed_map(&mut self) -> IndexMap<String, IndexMap<String, String>> {
        self.parsed_map.clone()
    }

    fn ensure_section(&mut self, parent: &String) {
        // check if the parent does not exist in the parsed dict
        self.parsed_map.entry(parent.to_string()).or_default();
    }

    fn add(&mut self, parent: &String, key: &String, value: &String) {
        // add the parent first
        self.ensure_section(parent);
        self.parsed_map
            .entry(parent.to_string())
            .and_modify(|info| {
                info.insert(key.to_string(), value.to_string());
            });
    }

    pub fn string(&mut self) -> String {
        let parsed_map = self.parsed_map.clone();
        let mut formatted_str = String::new();

        for (parent, info) in parsed_map.into_iter() {
            formatted_str.push_str(&format!("[{}]\n", parent));
            for (key, value) in info {
                formatted_str.push_str(&format!("{} = {}\n", key, value));
            }

            formatted_str.push('\n');
        }

        formatted_str.trim_end().to_string()
    }

    pub fn from_file(&mut self, file_name: String) -> std::io::Result<()> {
        let mut file = File::open(file_name)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;
        self.from_string(buffer)
    }

    pub fn save_to_file(&mut self, file_name: String) -> std::io::Result<()> {
        let mut buffer = File::create(file_name)?;
        buffer.write_all(self.string().as_bytes())
    }

    pub fn from_string(&mut self, content: String) -> Result<(), Error> {
        self.parsed_map = IndexMap::new();
        let mut key = String::new();
        let mut val = String::new();
        let mut section = String::new();

        // for parents
        let mut new_section = false;

        // read content lines
        for content_line in content.lines() {
            let line = &(content_line.replace(['\n', '\r'], ""));

            if !line.is_empty() {
                // parse sections
                if line.starts_with('[') && line.chars().nth(line.len() - 1).unwrap() == ']' {
                    // check number of opened and closed sections []
                    if line.matches('[').count() == 1 && line.matches(']').count() == 1 {
                        section = line[1..line.len() - 1].to_string();
                        self.ensure_section(&section);
                        new_section = true;
                    } else {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "invalid section! please make sure that you have one '[' and one ']'",
                        ));
                    }
                }
                // parse sections values
                else if new_section
                    && line.matches('=').count() == 1
                    && !["", "=", " "].contains(&&line[..1])
                    && !["", "=", " "].contains(&&line[line.len() - 1..])
                {
                    if line.contains(" = ") {
                        let splitted: Vec<&str> = line.split(" = ").collect();
                        key = splitted[0].to_string();
                        val = splitted[1].to_string();
                    } else if line.contains('=') {
                        let splitted: Vec<&str> = line.split('=').collect();
                        key = splitted[0].to_string();
                        val = splitted[1].to_string();
                    }

                    self.add(&section, &key, &val);
                }
                // parse comment lines
                else if line.starts_with(';') {
                    continue;
                }
                // invalid content
                else {
                    return Err(Error::new(ErrorKind::Other, "invalid ini content"));
                }
            } else if line.trim().is_empty() {
                continue;
            }
            // invalid content
            else {
                return Err(Error::new(ErrorKind::Other, "invalid ini content"));
            }
        }

        Ok(())
    }

    pub fn get_sections(&mut self) -> Vec<String> {
        let parsed_map = self.parsed_map.clone();
        parsed_map.keys().cloned().collect::<Vec<String>>()
    }

    pub fn get_section(&mut self, section_key: &str) -> Result<IndexMap<String, String>, Error> {
        if !self.parsed_map.contains_key(section_key) {
            return Err(Error::new(
                ErrorKind::Other,
                format!("section {} does not exist", section_key),
            ));
        }
        Ok(self.parsed_map.get(section_key).unwrap().clone())
    }

    pub fn get_options(&mut self, section_key: &str) -> Result<Vec<String>, Error> {
        let section = self.get_section(section_key)?;
        Ok(section.keys().cloned().collect::<Vec<String>>())
    }

    pub fn get_option(&mut self, section_key: &str, option_key: &str) -> Result<String, Error> {
        let section = self.get_section(section_key)?;

        if !section.contains_key(option_key) {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "option {} does not exist in section {}",
                    option_key, section_key
                ),
            ));
        }

        Ok(section.get(option_key).unwrap().to_string())
    }

    pub fn set_option(
        &mut self,
        section_key: &str,
        option_key: &str,
        option_val: &str,
    ) -> Result<(), Error> {
        let mut section = self.get_section(section_key)?;

        *section
            .entry(option_key.to_string())
            .or_insert(option_val.to_string()) = option_val.to_string();
        *self.parsed_map.get_mut(section_key).unwrap() = section;

        Ok(())
    }

    pub fn get_bool(&mut self, section_key: &str, option_key: &str) -> Result<bool, Error> {
        let option = self.get_option(section_key, option_key)?;

        if ["true", "True", "yes", "1"].contains(&option.as_str()) {
            return Ok(true);
        } else if ["false", "False", "no", "0"].contains(&option.as_str()) {
            return Ok(false);
        }

        return Err(Error::new(
            ErrorKind::Other,
            format!("option {} is not supported as a boolean", option),
        ));
    }

    pub fn get_int(&mut self, section_key: &str, option_key: &str) -> Result<u64, Error> {
        let option = self.get_option(section_key, option_key)?;
        Ok(option.parse::<u64>().unwrap())
    }

    pub fn get_float(&mut self, section_key: &str, option_key: &str) -> Result<f64, Error> {
        let option = self.get_option(section_key, option_key)?;
        Ok(option.parse::<f64>().unwrap())
    }
}
