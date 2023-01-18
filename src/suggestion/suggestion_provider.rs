use crate::{context::command_context::CommandContext, error::BrigadierError};

use super::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder};

pub trait SuggestionProvider<S> {
    fn get_suggestions(
        &self,
        context: CommandContext<S>,
        builder: &mut SuggestionsBuilder,
    ) -> Result<Suggestions, BrigadierError>;
}
