use std::fmt::Display;

use crate::{error::BrigadierError, string_reader::StringReader};

use super::ArgumentType;

#[allow(dead_code)]
pub enum StringType {
    Word,
    Quotable,
    Greedy,
}

impl StringType {
    fn get_examples(&self) -> Vec<String> {
        match self {
            Self::Word => vec!["word".to_string(), "word_with_underscores".to_string()],
            Self::Quotable => vec![
                "\"quoted phrase\"".to_string(),
                "word".to_string(),
                "\"\"".to_string(),
            ],
            Self::Greedy => vec![
                "word".to_string(),
                "word with spaces".to_string(),
                "\"and symbols\"".to_string(),
            ],
        }
    }
}

pub struct StringArgumentType {
    string_type: StringType,
}

#[allow(dead_code)]
impl StringArgumentType {
    pub fn new(string_type: StringType) -> Self {
        Self { string_type }
    }

    pub fn get_type(&self) -> &StringType {
        &self.string_type
    }
}

impl<S> ArgumentType<S> for StringArgumentType {
    type Type = String;

    fn parse(&self, reader: &mut StringReader) -> Result<Self::Type, BrigadierError> {
        match self.string_type {
            StringType::Word => reader.read_unqoted_string(),
            StringType::Quotable => reader.read_string(),
            StringType::Greedy => {
                let text = reader.get_remaining().to_owned();
                let total_length = reader.get_total_length();
                reader.set_cursor(total_length);
                Ok(text)
            }
        }
    }

    fn get_examples(&self) -> Vec<String> {
        self.string_type.get_examples()
    }
}

impl Display for StringArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "string()")
    }
}
