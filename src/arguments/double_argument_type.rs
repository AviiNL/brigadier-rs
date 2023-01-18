use std::fmt::Display;

use crate::error::BrigadierError;

use super::ArgumentType;

pub struct DoubleArgumentType {
    min: f64,
    max: f64,
}

#[allow(dead_code)]
impl DoubleArgumentType {
    pub fn new() -> Self {
        Self {
            min: f64::MIN,
            max: f64::MAX,
        }
    }

    pub fn with_min(self, min: f64) -> Self {
        Self { min, ..self }
    }

    pub fn with_range(self, min: f64, max: f64) -> Self {
        Self { min, max }
    }
}

impl<S> ArgumentType<S> for DoubleArgumentType {
    type Type = f64;

    fn parse(
        &self,
        reader: &mut crate::string_reader::StringReader,
    ) -> Result<Self::Type, BrigadierError> {
        let cursor = reader.get_cursor();
        let result = reader.read_double()?;

        if result < self.min {
            reader.set_cursor(cursor);
            return Err(BrigadierError::DoubleTooLow {
                result,
                min: self.min,
            });
        } else if result > self.max {
            reader.set_cursor(cursor);
            return Err(BrigadierError::DoubleTooHigh {
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

impl Display for DoubleArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.min == -f64::MIN && self.max == f64::MAX {
            write!(f, "double")
        } else if self.max == f64::MAX {
            write!(f, "double({})", self.min)
        } else {
            write!(f, "double({}, {})", self.min, self.max)
        }
    }
}
