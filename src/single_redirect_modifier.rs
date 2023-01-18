use crate::{context::command_context::CommandContext, error::BrigadierError};

pub trait SingleRedirectModifier<S> {
    fn apply(&self, context: CommandContext<S>) -> Result<S, BrigadierError>;
}
