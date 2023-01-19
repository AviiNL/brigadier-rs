use std::fmt::Display;

use super::command_node::CommandNode;

pub struct RootCommandNode<S> {
    _marker: std::marker::PhantomData<S>,
}

impl<S> RootCommandNode<S> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S> CommandNode<S> for RootCommandNode<S> {
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

    fn list_suggestions(
        &self,
        context: &mut crate::context::command_context::CommandContext<S>,
        builder: &mut crate::suggestion::suggestions_builder::SuggestionsBuilder,
    ) -> crate::suggestion::suggestions::Suggestions {
        todo!()
    }

    fn get_name(&self) -> &str {
        todo!()
    }

    fn get_usage(&self) -> &str {
        todo!()
    }

    fn get_command(&self) -> Option<Box<crate::command::Command<S>>> {
        todo!()
    }

    fn get_children(&self) -> Vec<Box<dyn CommandNode<S>>> {
        todo!()
    }

    fn get_child(&self, name: &str) -> Option<Box<dyn CommandNode<S>>> {
        todo!()
    }

    fn add_child(&mut self, child: Box<dyn CommandNode<S>>) {
        todo!()
    }

    fn get_redirect(&self) -> Option<Box<dyn CommandNode<S>>> {
        todo!()
    }

    fn get_redirect_modifier(
        &self,
    ) -> Option<Box<dyn crate::redirect_modifier::RedirectModifier<S>>> {
        todo!()
    }

    fn parse(
        &self,
        reader: &mut crate::string_reader::StringReader,
        context: &mut crate::context::command_context::CommandContext<S>,
    ) -> Result<(), String> {
        todo!()
    }
}

impl<S> Display for RootCommandNode<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<root>")
    }
}
