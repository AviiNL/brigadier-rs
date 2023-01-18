use std::fmt::Display;

use linked_hash_map::LinkedHashMap;

use crate::{
    command::Command,
    context::command_context::CommandContext,
    redirect_modifier::RedirectModifier,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};

use super::{
    command_node::{CommandName, CommandNode, CommandUsage},
    literal_command_node::LiteralCommandNode,
};

pub struct RootCommandNode<S> {
    _marker: std::marker::PhantomData<S>,
}

impl<S> Display for RootCommandNode<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<root>")
    }
}
