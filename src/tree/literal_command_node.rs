use linked_hash_map::LinkedHashMap;

use crate::{command::Command, redirect_modifier::RedirectModifier};

use super::{argument_command_node::ArgumentCommandNode, command_node::CommandNode};

pub struct LiteralCommandNode<S> {
    literal: String,
    literal_lower_case: String,
    _marker: std::marker::PhantomData<S>,
}

impl<S> LiteralCommandNode<S> {
    pub fn new(literal: String, command: Command<S>, requirement: Box<dyn Fn(&S) -> bool>) -> Self {
        Self {
            literal,
            literal_lower_case: literal.to_lowercase(),
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        let context_builder = CommandContextBuilder::new();
    }
}
