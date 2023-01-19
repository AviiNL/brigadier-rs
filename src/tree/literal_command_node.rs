use linked_hash_map::LinkedHashMap;

use crate::{command::Command, redirect_modifier::RedirectModifier};

use super::{argument_command_node::ArgumentCommandNode, command_node::CommandNode};

pub struct LiteralCommandNode<S> {
    literal: String,
    literal_lower_case: String,
    _marker: std::marker::PhantomData<S>,
}

impl<S> LiteralCommandNode<S> {
    pub fn new(
        literal: String,
        command: Option<Box<Command<S>>>,
        requirement: Box<dyn Fn(&S) -> bool>,
    ) -> Self {
        Self {
            literal: literal.clone(),
            literal_lower_case: literal.clone().to_lowercase(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S> CommandNode<S> for LiteralCommandNode<S> {
    fn can_use(&self, source: &S) -> bool {
        todo!()
    }

    fn find_ambiguities(
        &self,
        finder: &mut Box<dyn crate::ambiguity_consumer::AmbiguityConsumer<S>>,
    ) {
        todo!()
    }

    fn is_valid_input(&self, input: &str) -> bool {
        todo!()
    }

    fn get_sorted_key(&self) -> &str {
        todo!()
    }

    fn get_relevant_nodes(&self, input: &str) -> Vec<Box<dyn CommandNode<S>>> {
        todo!()
    }

    fn is_fork(&self) -> bool {
        todo!()
    }

    fn get_examples(&self) -> Vec<String> {
        todo!()
    }

    fn get_command(&self) -> Option<Box<Command<S>>> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        // let context_builder = CommandContextBuilder::new();
    }
}
