use crate::{error::BrigadierError, string_reader::StringReader};

const ARGUMENT_SEPARATOR: &str = " ";
const USAGE_OPTIONAL_OPEN: &str = "[";
const USAGE_OPTIONAL_CLOSE: &str = "]";
const USAGE_REQUIRED_OPEN: &str = "<";
const USAGE_REQUIRED_CLOSE: &str = ">";
const USAGE_OR: &str = "|";

pub struct CommandDispatcher<S> {
    // phantom marker for now
    _source: std::marker::PhantomData<S>,
}

impl<S> CommandDispatcher<S> {
    pub fn new() -> CommandDispatcher<S> {
        CommandDispatcher {
            _source: std::marker::PhantomData,
        }
    }

    pub fn execute<'a>(
        &self,
        command: impl Into<StringReader<'a>>,
        source: S,
    ) -> Result<usize, BrigadierError> {
        let string_reader: StringReader = command.into();

        // let parsed_result = self.parse(string_reader)?;

        println!("{:?}", string_reader.get_string());

        Ok(0)
    }
}
