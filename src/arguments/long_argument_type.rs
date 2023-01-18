use std::fmt::Display;

use crate::error::BrigadierError;

use super::ArgumentType;

pub struct LongArgumentType {
    min: i64,
    max: i64,
}

#[allow(dead_code)]
impl LongArgumentType {
    pub fn new() -> Self {
        Self {
            min: i64::MIN,
            max: i64::MAX,
        }
    }

    pub fn with_min(self, min: i64) -> Self {
        Self { min, ..self }
    }

    // todo: add safety range check? min <= max
    pub fn with_range(self, min: i64, max: i64) -> Self {
        Self { min, max }
    }
}

impl<S> ArgumentType<S> for LongArgumentType {
    type Type = i64;

    fn parse(
        &self,
        reader: &mut crate::string_reader::StringReader,
    ) -> Result<Self::Type, BrigadierError> {
        let cursor = reader.get_cursor();
        let result = reader.read_long()?;

        if result < self.min {
            reader.set_cursor(cursor);
            return Err(BrigadierError::LongTooLow {
                result,
                min: self.min,
            });
        } else if result > self.max {
            reader.set_cursor(cursor);
            return Err(BrigadierError::LongTooHigh {
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

impl Display for LongArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.min == i64::MIN && self.max == i64::MAX {
            write!(f, "longArg")
        } else if self.max == i64::MAX {
            write!(f, "longArg({})", self.min)
        } else {
            write!(f, "longArg({}, {})", self.min, self.max)
        }
    }
}
