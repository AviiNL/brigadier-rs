use std::fmt::Display;

use crate::context::string_range::StringRange;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Suggestion {
    range: StringRange,
    text: String,
    tooltip: Option<String>,
}

impl Suggestion {
    pub fn new(range: StringRange, text: String) -> Self {
        Self {
            range,
            text,
            tooltip: None,
        }
    }

    pub fn with_tooltip(self, tooltip: impl Into<String>) -> Self {
        Self {
            tooltip: Some(tooltip.into()),
            ..self
        }
    }

    pub fn get_range(&self) -> &StringRange {
        &self.range
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_tooltip(&self) -> Option<&str> {
        self.tooltip.as_deref()
    }

    pub fn apply(&self, input: &str) -> String {
        if self.range.get_start() == 0 && self.range.get_end() == input.len() {
            return self.text.to_owned();
        }

        let mut result = String::new();
        if self.range.get_start() > 0 {
            result.push_str(&input[..self.range.get_start()]);
        }
        result.push_str(&self.text);
        if self.range.get_end() < input.len() {
            result.push_str(&input[self.range.get_end()..]);
        }

        result
    }

    pub fn expand(&self, command: &str, range: StringRange) -> Self {
        if range == self.range {
            return self.clone();
        }

        let mut result = String::new();
        if range.get_start() < self.range.get_start() {
            result.push_str(&command[range.get_start()..self.range.get_start()]);
        }
        result.push_str(&self.text);
        if range.get_end() > self.range.get_end() {
            result.push_str(&command[self.range.get_end()..range.get_end()]);
        }

        Self {
            range,
            text: result,
            tooltip: self.tooltip.clone(),
        }
    }
}

impl Display for Suggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tooltip = match &self.tooltip {
            Some(tooltip) => tooltip,
            None => "null",
        };

        write!(
            f,
            "Suggestion{{range={}, text={}, tooltip={}}}",
            self.range, self.text, tooltip
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn apply_insertation_start() {
        let suggestion = Suggestion::new(StringRange::at(0), "And so I said: ".to_owned());
        assert_eq!(
            suggestion.apply("Hello World!"),
            "And so I said: Hello World!"
        );
    }

    #[test]
    fn apply_insertation_middle() {
        let suggestion = Suggestion::new(StringRange::at(6), "small ".to_owned());
        assert_eq!(suggestion.apply("Hello World!"), "Hello small World!");
    }

    #[test]
    fn apply_insertation_end() {
        let suggestion = Suggestion::new(StringRange::at(5), " world!".to_owned());
        assert_eq!(suggestion.apply("Hello"), "Hello world!");
    }

    #[test]
    fn apply_replacement_start() {
        let suggestion = Suggestion::new(StringRange::between(0, 5), "Goodbye".to_owned());
        assert_eq!(suggestion.apply("Hello World!"), "Goodbye World!");
    }

    #[test]
    fn apply_replacement_middle() {
        let suggestion = Suggestion::new(StringRange::between(6, 11), "Alex".to_owned());
        assert_eq!(suggestion.apply("Hello World!"), "Hello Alex!");
    }

    #[test]
    fn apply_replacement_end() {
        let suggestion = Suggestion::new(StringRange::between(6, 12), "Creeper!".to_owned());
        assert_eq!(suggestion.apply("Hello world!"), "Hello Creeper!");
    }

    #[test]
    fn apply_replacement_everything() {
        let suggestion = Suggestion::new(StringRange::between(0, 12), "Oh dear.".to_owned());
        assert_eq!(suggestion.apply("Hello world!"), "Oh dear.");
    }

    #[test]
    fn expand_unchanged() {
        let suggestion = Suggestion::new(StringRange::at(1), "oo".to_owned());
        assert_eq!(suggestion.expand("f", StringRange::at(1)), suggestion);
    }

    #[test]
    fn expand_left() {
        let suggestion = Suggestion::new(StringRange::at(1), "oo".to_owned());
        assert_eq!(
            suggestion.expand("f", StringRange::between(0, 1)),
            Suggestion::new(StringRange::between(0, 1), "foo".to_owned())
        );
    }

    #[test]
    fn expand_right() {
        let suggestion = Suggestion::new(StringRange::at(0), "minecraft:".to_owned());

        assert_eq!(
            suggestion.expand("fish", StringRange::between(0, 4)),
            Suggestion::new(StringRange::between(0, 4), "minecraft:fish".to_owned())
        );
    }

    #[test]
    fn expand_both() {
        let suggestion = Suggestion::new(StringRange::at(11), "minecraft:".to_owned());

        assert_eq!(
            suggestion.expand("give Steve fish_block", StringRange::between(5, 21)),
            Suggestion::new(
                StringRange::between(5, 21),
                "Steve minecraft:fish_block".to_owned()
            )
        );
    }

    #[test]
    fn expand_replacement() {
        let suggestion = Suggestion::new(StringRange::between(6, 11), "strangers".to_owned());

        assert_eq!(
            suggestion.expand("Hello world!", StringRange::between(0, 12)),
            Suggestion::new(StringRange::between(0, 12), "Hello strangers!".to_owned())
        );
    }
}
