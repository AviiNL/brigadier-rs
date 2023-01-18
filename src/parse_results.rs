pub struct ParseResults<S> {
    context: CommandContextBuilder<S>,
    reader: StringReader,
}

impl<S> ParseResults<S> {
    pub fn new(context: CommandContextBuilder<S>, string_reader: StringReader) -> ParseResults<S> {
        ParseResults {
            context,
            reader: string_reader,
        }
    }

    pub fn get_reader(&self) -> &StringReader {
        &self.reader
    }

    pub fn get_context(&self) -> &CommandContextBuilder<S> {
        &self.context
    }
}
