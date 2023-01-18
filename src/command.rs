use crate::{context::command_context::CommandContext, error::BrigadierError};

pub const SINGLE_SUCCESS: i32 = 1;

pub trait Command<S> {
    fn run(&self, context: CommandContext<S>) -> Result<i32, BrigadierError>;
}
