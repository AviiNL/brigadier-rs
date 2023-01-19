use crate::{
    ambiguity_consumer::AmbiguityConsumer,
    builder::argument_builder::ArgumentBuilder,
    command::Command,
    context::command_context::CommandContext,
    redirect_modifier::RedirectModifier,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};

pub trait CommandNode<S> {
    fn can_use(&self, source: &S) -> bool;

    fn find_ambiguities(&self, finder: &mut Box<dyn AmbiguityConsumer<S>>);

    fn is_valid_input(&self, input: &str) -> bool;

    fn get_sorted_key(&self) -> &str;
    fn get_relevant_nodes(&self, input: &str) -> Vec<Box<dyn CommandNode<S>>>;
    fn is_fork(&self) -> bool;
    fn get_examples(&self) -> Vec<String>;
    fn list_suggestions(
        &self,
        context: &mut CommandContext<S>,
        builder: &mut SuggestionsBuilder,
    ) -> Suggestions;

    fn get_name(&self) -> &str;
    fn get_usage(&self) -> &str;
    fn get_command(&self) -> Option<Box<Command<S>>>;
    fn get_children(&self) -> Vec<Box<dyn CommandNode<S>>>;
    fn get_child(&self, name: &str) -> Option<Box<dyn CommandNode<S>>>;
    fn add_child(&mut self, child: Box<dyn CommandNode<S>>);
    fn create_builder(&self) -> Box<dyn ArgumentBuilder<S>>;
    fn get_redirect(&self) -> Option<Box<dyn CommandNode<S>>>;
    fn get_redirect_modifier(&self) -> Option<Box<dyn RedirectModifier<S>>>;
    fn parse(
        &self,
        reader: &mut StringReader,
        context: &mut CommandContext<S>,
    ) -> Result<(), String>;
}
