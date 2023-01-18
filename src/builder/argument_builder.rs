use crate::{
    command::Command, redirect_modifier::RedirectModifier,
    single_redirect_modifier::SingleRedirectModifier, tree::command_node::CommandNode,
};

pub trait ArgumentBuilder<S, T>
where
    T: ArgumentBuilder<S, T>,
{
    fn then(self, argument: T) -> T;
    fn then_command_node(self, argument: Box<dyn CommandNode<S>>) -> T;
    fn get_arguments(&self) -> Vec<Box<dyn CommandNode<S>>>;
    fn executes(self, command: Box<dyn Command<S>>) -> T;
    fn get_command(&self) -> Option<Box<dyn Command<S>>>;
    fn requires(self, requirement: Box<dyn Fn(&S) -> bool>) -> T;
    fn get_requirement(&self) -> Option<Box<dyn Fn(&S) -> bool>>;
    fn redirect(self, target: Box<dyn CommandNode<S>>) -> T;
    fn redirect_with_modifier(
        self,
        target: Box<dyn CommandNode<S>>,
        modifier: Box<dyn SingleRedirectModifier<S>>,
    ) -> T;
    fn fork(&self, target: Box<dyn CommandNode<S>>, modifier: Box<dyn RedirectModifier<S>>) -> T;
    fn forward(
        self,
        target: Box<dyn CommandNode<S>>,
        modifier: Box<dyn RedirectModifier<S>>,
        fork: bool,
    ) -> T;
    fn get_redirect(&self) -> Option<Box<dyn CommandNode<S>>>;
    fn get_redirect_modifier(&self) -> Option<Box<dyn RedirectModifier<S>>>;
    fn is_fork(&self) -> bool;
    fn build(&self) -> Box<dyn CommandNode<S>>;
}
