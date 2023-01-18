use std::fmt::Display;

use crate::error::BrigadierError;

use super::ArgumentType;

pub struct IntegerArgumentType {
    min: i32,
    max: i32,
}

#[allow(dead_code)]
impl IntegerArgumentType {
    pub fn new() -> Self {
        Self {
            min: i32::MIN,
            max: i32::MAX,
        }
    }

    pub fn with_min(self, min: i32) -> Self {
        Self { min, ..self }
    }

    // todo: add safety range check? min <= max
    pub fn with_range(self, min: i32, max: i32) -> Self {
        Self { min, max }
    }
}

impl<S> ArgumentType<S> for IntegerArgumentType {
    type Type = i32;

    fn parse(
        &self,
        reader: &mut crate::string_reader::StringReader,
    ) -> Result<Self::Type, BrigadierError> {
        let cursor = reader.get_cursor();
        let result = reader.read_int()?;

        if result < self.min {
            reader.set_cursor(cursor);
            return Err(BrigadierError::IntegerTooLow {
                result,
                min: self.min,
            });
        } else if result > self.max {
            reader.set_cursor(cursor);
            return Err(BrigadierError::IntegerTooHigh {
                result,
                max: self.max,
            });
        }

        return Ok(result);
    }

    fn get_examples(&self) -> Vec<String> {
        vec!["0".to_string(), "123".to_string(), "-123".to_string()]
    }
}

impl Display for IntegerArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.min == i32::MIN && self.max == i32::MAX {
            write!(f, "integer")
        } else if self.max == i32::MAX {
            write!(f, "integer({})", self.min)
        } else {
            write!(f, "integer({}, {})", self.min, self.max)
        }
    }
}
