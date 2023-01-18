use crate::{context::command_context::CommandContext, error::BrigadierError};

pub trait RedirectModifier<S> {
    fn apply(&self, context: CommandContext<S>) -> Result<Vec<S>, BrigadierError>;
}
