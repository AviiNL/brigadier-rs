use std::{any::Any, collections::HashMap};

use crate::{
    arguments::{
        BoolArgumentType, DoubleArgumentType, FloatArgumentType, IntegerArgumentType,
        LongArgumentType, StringArgumentType,
    },
    command::Command,
};

use super::{parsed_argument::ParsedArgument, string_range::StringRange};

pub enum ArgumentType {
    Bool(BoolArgumentType),
    Double(DoubleArgumentType),
    Float(FloatArgumentType),
    Integer(IntegerArgumentType),
    Long(LongArgumentType),
    String(StringArgumentType),
}

pub struct CommandContext<S> {
    source: S,
    input: String,
    command: Box<Command<S>>,
    arguments: HashMap<String, ParsedArgument<S, ArgumentType>>,
    // root_node: CommandNode<S>,
    // nodes: Vec<ParsedCommandNode<S>>,
    range: StringRange,
    // child: Option<Box<CommandContext<S>>>,
    // modifier: RedirectModifier<S>,
    forks: bool,
}
