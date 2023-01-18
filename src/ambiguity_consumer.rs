use crate::tree::command_node::CommandNode;

pub trait AmbiguityConsumer<S> {
    fn ambiguous(
        &mut self,
        parent: dyn CommandNode<S>,
        child: dyn CommandNode<S>,
        sibling: dyn CommandNode<S>,
        inputs: Vec<String>,
    );
}
