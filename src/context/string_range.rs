use std::fmt::Display;

use crate::string_reader::StringReader;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StringRange {
    start: usize,
    end: usize,
}

impl StringRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn at(pos: usize) -> Self {
        Self::new(pos, pos)
    }

    pub fn between(start: usize, end: usize) -> Self {
        Self::new(start, end)
    }

    pub fn encompassing(range_a: &Self, range_b: &Self) -> Self {
        Self::new(
            std::cmp::min(range_a.start, range_b.start),
            std::cmp::max(range_a.end, range_b.end),
        )
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_end(&self) -> usize {
        self.end
    }

    pub fn get_from_string_reader(&self, reader: StringReader) -> String {
        reader.get_string()[self.start..self.end].to_string()
    }

    pub fn get_from_string(&self, input: &str) -> String {
        input[self.start..self.end].to_string()
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn get_length(&self) -> usize {
        self.end - self.start
    }
}

impl Display for StringRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StringRange{{start={}, end={}}}", self.start, self.end)
    }
}
