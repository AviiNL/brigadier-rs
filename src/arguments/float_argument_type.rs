use std::fmt::Display;

use crate::error::BrigadierError;

use super::ArgumentType;

pub struct FloatArgumentType {
    min: f32,
    max: f32,
}

#[allow(dead_code)]
impl FloatArgumentType {
    pub fn new() -> Self {
        Self {
            min: f32::MIN,
            max: f32::MAX,
        }
    }

    pub fn with_min(self, min: f32) -> Self {
        Self { min, ..self }
    }

    pub fn with_range(self, min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

impl<S> ArgumentType<S> for FloatArgumentType {
    type Type = f32;

    fn parse(
        &self,
        reader: &mut crate::string_reader::StringReader,
    ) -> Result<Self::Type, BrigadierError> {
        let cursor = reader.get_cursor();
        let result = reader.read_float()?;

        if result < self.min {
            reader.set_cursor(cursor);
            return Err(BrigadierError::FloatTooLow {
                result,
                min: self.min,
            });
        } else if result > self.max {
            reader.set_cursor(cursor);
            return Err(BrigadierError::FloatTooHigh {
                result,
                max: self.max,
            });
        }

        return Ok(result);
    }

    fn get_examples(&self) -> Vec<String> {
        vec![
            "0".to_string(),
            "1.2".to_string(),
            ".5".to_string(),
            "-1".to_string(),
            "-.5".to_string(),
            "-1234.56".to_string(),
        ]
    }
}

impl Display for FloatArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.min == -f32::MIN && self.max == f32::MAX {
            write!(f, "float")
        } else if self.max == f32::MAX {
            write!(f, "float({})", self.min)
        } else {
            write!(f, "float({}, {})", self.min, self.max)
        }
    }
}
