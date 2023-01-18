use std::fmt::{Display, Formatter};

use crate::{
    context::command_context::CommandContext, error::BrigadierError,
    suggestion::suggestions_builder::SuggestionsBuilder,
};

use super::ArgumentType;

pub struct BoolArgumentType;

impl<S> ArgumentType<S> for BoolArgumentType {
    type Type = bool;

    fn parse(
        &self,
        reader: &mut crate::string_reader::StringReader,
    ) -> Result<Self::Type, BrigadierError> {
        return reader.read_boolean();
    }

    #[allow(unused_variables)]
    fn list_suggestions(
        &self,
        context: CommandContext<S>,
        builder: &mut SuggestionsBuilder,
    ) -> super::Suggestions {
        if "true".starts_with(builder.get_remaining_lower_case()) {
            builder.suggest("true");
        }
        if "false".starts_with(builder.get_remaining_lower_case()) {
            builder.suggest("false");
        }
        return builder.build(); // TODO: build_future()
    }

    fn get_examples(&self) -> Vec<String> {
        vec!["true".to_string(), "false".to_string()]
    }
}

impl Display for BoolArgumentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "boolean()")
    }
}
