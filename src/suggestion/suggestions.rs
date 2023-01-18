use std::{collections::HashSet, fmt::Display};

use crate::context::string_range::StringRange;

use super::suggestion::Suggestion;

pub struct Suggestions {
    pub(crate) suggestions: Vec<Suggestion>,
    pub(crate) range: StringRange,
}

impl Suggestions {
    pub fn new(range: StringRange, suggestions: Vec<Suggestion>) -> Self {
        Self { range, suggestions }
    }

    pub fn empty() -> Self {
        Self {
            range: StringRange::at(0),
            suggestions: Vec::new(),
        }
    }

    pub fn get_range(&self) -> &StringRange {
        &self.range
    }

    pub fn get_list(&self) -> &[Suggestion] {
        &self.suggestions
    }

    pub fn is_empty(&self) -> bool {
        self.suggestions.is_empty()
    }

    pub fn merge(command: &str, input: Vec<Suggestions>) -> Self {
        if input.is_empty() {
            return Self::empty();
        } else if input.len() == 1 {
            return input.into_iter().next().unwrap();
        }

        let mut texts: HashSet<Suggestion> = HashSet::new();
        for suggestions in input {
            for suggestion in suggestions.get_list() {
                texts.insert(suggestion.clone());
            }
        }

        Self::create(command, texts)
    }

    pub fn create(command: &str, suggestions: HashSet<Suggestion>) -> Self {
        if suggestions.is_empty() {
            return Self::empty();
        }

        let mut start = i32::MAX;
        let mut end = i32::MIN;

        for suggestion in &suggestions {
            start = std::cmp::min(suggestion.get_range().get_start() as i32, start);
            end = std::cmp::max(suggestion.get_range().get_end() as i32, end);
        }
        let range = StringRange::between(start as usize, end as usize);
        let mut texts = HashSet::new();
        for suggestion in &suggestions {
            texts.insert(suggestion.expand(command, range));
        }
        let mut sorted = Vec::new();
        for suggestion in texts {
            sorted.push(suggestion);
        }
        sorted.sort();
        Self::new(range, sorted)
    }
}

impl Display for Suggestions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut suggestions = String::new();

        for suggestion in &self.suggestions {
            suggestions.push_str(&format!("{}, ", suggestion));
        }

        // trim off the last ", " ?
        suggestions.truncate(suggestions.len() - 2);

        write!(
            f,
            "Suggestions{{range={}, suggestions=[{}]}}",
            self.range, suggestions
        )
    }
}
