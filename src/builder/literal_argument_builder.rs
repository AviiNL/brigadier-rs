use crate::{
    command::Command,
    redirect_modifier::RedirectModifier,
    tree::{
        command_node::CommandNode, literal_command_node::LiteralCommandNode,
        root_command_node::RootCommandNode,
    },
};

pub struct LiteralArgumentBuilder<S> {
    pub(crate) literal: String,
    pub(crate) arguments: RootCommandNode<S>,
    pub(crate) command: Option<Command<S>>,
    pub(crate) requirement: Option<Box<dyn Fn(&S) -> bool>>,
    pub(crate) target: Option<Box<dyn CommandNode<S>>>,
    pub(crate) modifier: Option<Box<dyn RedirectModifier<S>>>,
    pub(crate) forks: bool,
}

impl<S> LiteralArgumentBuilder<S> {
    pub fn new(literal: String) -> Self {
        Self {
            literal,
            arguments: RootCommandNode::new(),
            command: None,
            requirement: None,
            target: None,
            modifier: None,
            forks: false,
        }
    }

    pub fn then(&mut self, argument: Box<dyn CommandNode<S>>) -> &mut Self {
        if self.target.is_some() {
            panic!("Cannot add children to a redirected node"); // todo: tracing error or smth
        }

        self.arguments.add_child(argument);
        self
    }

    pub fn get_arguments(&self) -> Vec<Box<dyn CommandNode<S>>> {
        self.arguments.get_children()
    }

    /// this.executes(..)
    pub fn with_command(&mut self, command: Command<S>) -> &mut Self {
        self.command = Some(command);
        self
    }

    pub fn with_requirement(&mut self, requirement: Box<dyn Fn(&S) -> bool>) -> &mut Self {
        self.requirement = Some(requirement);
        self
    }

    pub fn build() -> LiteralCommandNode<S> {
        todo!()
    }
}
