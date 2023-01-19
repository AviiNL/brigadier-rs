use crate::{context::command_context::CommandContext, error::BrigadierError};

pub const SINGLE_SUCCESS: i32 = 1;

//pub trait Command<S>: 'static + Send + Sync + Clone {
//    fn run(&self, context: CommandContext<S>) -> Result<i32, BrigadierError>;
//}

pub struct Command<S>(fn(context: CommandContext<S>) -> Result<i32, BrigadierError>);

impl<S> Command<S> {
    pub fn new(f: fn(context: CommandContext<S>) -> Result<i32, BrigadierError>) -> Self {
        Self(f)
    }

    pub fn run(&self, context: CommandContext<S>) -> Result<i32, BrigadierError> {
        (self.0)(context)
    }
}

impl<S> Copy for Command<S> {}

impl<S> Clone for Command<S> {
    fn clone(&self) -> Self {
        *self
    }
}
