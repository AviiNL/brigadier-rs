use linked_hash_map::LinkedHashMap;

use crate::{
    arguments::ArgumentType, command::Command, redirect_modifier::RedirectModifier,
    suggestion::suggestion_provider::SuggestionProvider,
};

use super::{command_node::CommandNode, literal_command_node::LiteralCommandNode};

pub struct ArgumentCommandNode<S, T> {
    children: LinkedHashMap<String, Box<dyn CommandNode<S>>>,
    literals: LinkedHashMap<String, LiteralCommandNode<S>>,
    arguments: LinkedHashMap<String, ArgumentCommandNode<S, T>>,
    requirement: Option<Box<dyn Fn(&S) -> bool>>,
    redirect: Option<Box<dyn CommandNode<S>>>,
    modifier: Option<Box<dyn RedirectModifier<S>>>,
    forks: bool,
    command: Box<Command<S>>,

    name: String,
    argument_type: Box<dyn ArgumentType<S, Type = T>>,
    custom_suggestions: Box<dyn SuggestionProvider<S>>,
}
