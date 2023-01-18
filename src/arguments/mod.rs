pub mod bool_argument_type;
pub mod double_argument_type;
pub mod float_argument_type;
pub mod integer_argument_type;
pub mod long_argument_type;
pub mod string_argument_type;

pub use bool_argument_type::BoolArgumentType;
pub use double_argument_type::DoubleArgumentType;
pub use float_argument_type::FloatArgumentType;
pub use integer_argument_type::IntegerArgumentType;
pub use long_argument_type::LongArgumentType;
pub use string_argument_type::StringArgumentType;

use std::fmt::Display;

use crate::{
    context::command_context::CommandContext,
    error::BrigadierError,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};

pub trait ArgumentType<S>: Display {
    type Type;

    fn parse(&self, reader: &mut StringReader) -> Result<Self::Type, BrigadierError>;

    #[allow(unused_variables)]
    fn list_suggestions(
        &self,
        context: CommandContext<S>,
        builder: &mut SuggestionsBuilder,
    ) -> Suggestions {
        Suggestions::empty()
    }

    fn get_examples(&self) -> Vec<String> {
        Vec::new()
    }
}
