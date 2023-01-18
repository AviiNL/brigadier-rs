use std::collections::HashSet;

use crate::context::string_range::StringRange;

use super::{suggestion::Suggestion, suggestions::Suggestions};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct SuggestionsBuilder {
    input: String,
    input_lower_case: String,
    start: usize,
    remaining: String,
    remaining_lower_case: String,
    result: HashSet<Suggestion>,
}

impl SuggestionsBuilder {
    pub fn new(input: &str, start: usize) -> Self {
        let input_lower_case = input.to_lowercase();
        Self {
            input: input.to_owned(),
            input_lower_case: input_lower_case.to_owned(),
            start,
            remaining: input.to_owned().split_off(start),
            remaining_lower_case: input_lower_case.to_owned().split_off(start),
            result: HashSet::new(),
        }
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_remaining(&self) -> &str {
        &self.remaining
    }

    pub fn get_remaining_lower_case(&self) -> &str {
        &self.remaining_lower_case
    }

    pub fn build(&self) -> Suggestions {
        Suggestions::create(&self.input, self.result.clone())
    }

    pub fn suggest(&mut self, text: &str) -> &mut Self {
        if text == self.remaining {
            return self;
        }

        self.result.insert(Suggestion::new(
            StringRange::between(self.start, self.input.len()),
            text.to_owned(),
        ));

        self
    }

    pub fn suggest_with_tooltip(&mut self, text: &str, tooltip: &str) -> &mut Self {
        if text == self.remaining {
            return self;
        }

        self.result.insert(
            Suggestion::new(
                StringRange::between(self.start, self.input.len()),
                text.to_owned(),
            )
            .with_tooltip(tooltip.to_owned()),
        );

        self
    }

    // TODO: suggest_integer, look at use-cases for it on how to implement
    // TODO: suggest_integer_with_tooltip

    pub fn add(&mut self, other: Self) -> &mut Self {
        self.result.extend(other.result);

        self
    }

    pub fn create_offset(&self, start: usize) -> Self {
        Self::new(&self.input, start)
    }

    pub fn restart(&self) -> Self {
        self.create_offset(self.start)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn suggest_appends() {
        let mut builder = SuggestionsBuilder::new("Hello w", 6);
        let result = builder.suggest("world!").build();
        assert_eq!(
            result.get_list(),
            &[Suggestion::new(
                StringRange::between(6, 7),
                "world!".to_owned()
            )]
        );
        assert_eq!(result.get_range(), &StringRange::between(6, 7));
        assert_eq!(result.is_empty(), false);
    }

    #[test]
    fn suggest_replaces() {
        let mut builder = SuggestionsBuilder::new("Hello w", 6);
        let result = builder.suggest("everybody").build();
        assert_eq!(
            result.get_list(),
            &[Suggestion::new(
                StringRange::between(6, 7),
                "everybody".to_owned()
            )]
        );
        assert_eq!(result.get_range(), &StringRange::between(6, 7));
        assert_eq!(result.is_empty(), false);
    }

    #[test]
    fn suggest_noop() {
        let mut builder = SuggestionsBuilder::new("Hello w", 6);
        let result = builder.suggest("w").build();
        assert_eq!(result.get_list(), &[]);
        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn suggest_multiple() {
        let mut builder = SuggestionsBuilder::new("Hello w", 6);
        let result = builder
            .suggest("world!")
            .suggest("everybody")
            .suggest("weekend")
            .build();

        // this is testing ordering, should be alphabetical
        assert_eq!(
            result.get_list(),
            &[
                Suggestion::new(StringRange::between(6, 7), "everybody".to_owned()),
                Suggestion::new(StringRange::between(6, 7), "weekend".to_owned()),
                Suggestion::new(StringRange::between(6, 7), "world!".to_owned()),
            ]
        );
        assert_eq!(result.get_range(), &StringRange::between(6, 7));
        assert_eq!(result.is_empty(), false);
    }

    #[test]
    fn restart() {
        let mut builder = SuggestionsBuilder::new("Hello w", 6);
        builder.suggest("won't be included in restart");
        let other = builder.restart();
        assert_ne!(builder, other);
        assert_eq!(other.get_input(), builder.get_input());
        assert_eq!(other.get_start(), builder.get_start());
        assert_eq!(other.get_remaining(), builder.get_remaining());
    }

    #[test]
    fn sort_alpgabetical() {
        let mut builder = SuggestionsBuilder::new("Hello w", 6);
        let result = builder
            .suggest("2")
            .suggest("4")
            .suggest("6")
            .suggest("8")
            .suggest("30")
            .suggest("32")
            .build();

        let actual = result
            .get_list()
            .iter()
            .map(Suggestion::get_text)
            .collect::<Vec<_>>();
        assert_eq!(actual, vec!["2", "30", "32", "4", "6", "8"]);
    }

    // Suggestions result = builder.suggest("2").suggest("4").suggest("6").suggest("8").suggest("30").suggest("32").build();
    // Suggestions result = builder.suggest(2).suggest(4).suggest(6).suggest(8).suggest(30).suggest(32).build();
}
