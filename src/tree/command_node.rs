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
}

pub trait CommandCommand<S> {
    fn get_command(&self) -> Box<dyn Command<S>>;
}

pub trait CommandChildren<S> {
    fn get_children(&self) -> Vec<Box<dyn CommandNode<S>>>;
    fn get_child(&self, name: &str) -> Option<Box<dyn CommandNode<S>>>;
    fn add_child(&mut self, child: Box<dyn CommandNode<S>>);
}

pub trait CommandRedirect<S> {
    fn get_redirect(&self) -> Option<Box<dyn CommandNode<S>>>;
    fn get_redirect_modifier(&self) -> Option<Box<dyn RedirectModifier<S>>>;
}

pub trait CommandName {
    fn get_name(&self) -> &str;
}

pub trait CommandUsage {
    fn get_usage(&self) -> &str;
}

pub trait CommandSuggestions<S> {
    fn list_suggestions(
        &self,
        context: &mut CommandContext<S>,
        builder: &mut SuggestionsBuilder,
    ) -> Suggestions;
}

pub trait ParsableCommand<S> {
    fn parse(
        &self,
        reader: &mut StringReader,
        context: &mut CommandContext<S>,
    ) -> Result<(), String>;
}

pub trait ComandBuilder<S, T>
where
    T: ArgumentBuilder<S, T>,
{
    fn create_builder(&self) -> Box<dyn ArgumentBuilder<S, T>>;
}
