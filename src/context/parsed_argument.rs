use crate::error::BrigadierError;

use super::string_range::StringRange;

// Why does this have S?

#[derive(Debug, PartialEq)]
pub struct ParsedArgument<S, T> {
    range: StringRange,
    result: Result<T, BrigadierError>,
    _marker: std::marker::PhantomData<S>,
}

impl<S, T> ParsedArgument<S, T> {
    pub fn new(start: usize, end: usize, result: Result<T, BrigadierError>) -> Self {
        let range = StringRange::new(start, end);
        Self {
            range,
            result,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get_range(&self) -> StringRange {
        self.range
    }

    pub fn get_result(&self) -> &Result<T, BrigadierError> {
        &self.result
    }
}
