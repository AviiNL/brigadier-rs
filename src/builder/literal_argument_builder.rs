use crate::{
    command::Command,
    tree::{command_node::CommandNode, literal_command_node::LiteralCommandNode},
};

use super::argument_builder::ArgumentBuilder;

pub struct LiteralArgumentBuilder<S> {
    pub literal: String,
    pub command: Option<Box<Command<S>>>,
}

impl<S> LiteralArgumentBuilder<S> {
    pub fn new(literal: String) -> Self {
        Self {
            literal,
            command: None,
        }
    }

    fn get_literal(&self) -> String {
        self.literal.clone()
    }
}

impl<S> ArgumentBuilder<S> for LiteralArgumentBuilder<S> {
    fn build(&self) -> Box<dyn CommandNode<S>> {
        let node = LiteralCommandNode::<S>::new(
            self.get_literal(),
            <LiteralArgumentBuilder<S> as ArgumentBuilder<S, Self>>::get_command(self),
            Box::new(|_| true),
        );

        Box::new(node)
    }

    fn then(self, argument: Self) -> Self {
        todo!()
    }

    fn then_command_node(
        self,
        argument: Box<dyn crate::tree::command_node::CommandNode<S>>,
    ) -> Self {
        todo!()
    }

    fn get_arguments(&self) -> Vec<Box<dyn crate::tree::command_node::CommandNode<S>>> {
        todo!()
    }

    fn executes(mut self, command: Box<Command<S>>) -> Self {
        self.command = Some(command);
        self
    }

    fn requires(self, requirement: Box<dyn Fn(&S) -> bool>) -> Self {
        todo!()
    }

    fn get_requirement(&self) -> Option<Box<dyn Fn(&S) -> bool>> {
        todo!()
    }

    fn redirect(self, target: Box<dyn crate::tree::command_node::CommandNode<S>>) -> Self {
        todo!()
    }

    fn redirect_with_modifier(
        self,
        target: Box<dyn crate::tree::command_node::CommandNode<S>>,
        modifier: Box<dyn crate::single_redirect_modifier::SingleRedirectModifier<S>>,
    ) -> Self {
        todo!()
    }

    fn fork(
        &self,
        target: Box<dyn crate::tree::command_node::CommandNode<S>>,
        modifier: Box<dyn crate::redirect_modifier::RedirectModifier<S>>,
    ) -> Self {
        todo!()
    }

    fn forward(
        self,
        target: Box<dyn crate::tree::command_node::CommandNode<S>>,
        modifier: Box<dyn crate::redirect_modifier::RedirectModifier<S>>,
        fork: bool,
    ) -> Self {
        todo!()
    }

    fn get_redirect(&self) -> Option<Box<dyn crate::tree::command_node::CommandNode<S>>> {
        todo!()
    }

    fn get_redirect_modifier(
        &self,
    ) -> Option<Box<dyn crate::redirect_modifier::RedirectModifier<S>>> {
        todo!()
    }

    fn is_fork(&self) -> bool {
        todo!()
    }
    fn get_command(&self) -> Option<Box<Command<S>>> {
        self.command.clone()
    }
}
