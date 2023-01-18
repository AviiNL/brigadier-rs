use crate::error::BrigadierError;

const SYNTAX_ESCAPE: char = '\\';
const SYNTAX_DOUBLE_QUOTE: char = '"';
const SYNTAX_SINGLE_QUOTE: char = '\'';

pub struct StringReader<'a> {
    pub string: &'a str,
    cursor: usize,
}

impl<'a> StringReader<'a> {
    pub fn new(string: &str) -> StringReader {
        StringReader { string, cursor: 0 }
    }

    pub fn get_string(&self) -> &str {
        // owned / clone string ?
        self.string
    }

    pub fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }

    pub fn get_remaining_length(&self) -> usize {
        self.string.len() - self.cursor
    }

    pub fn get_total_length(&self) -> usize {
        self.string.len()
    }

    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    pub fn get_read(&self) -> &str {
        &self.string[..self.cursor]
    }

    pub fn get_remaining(&self) -> &str {
        &self.string[self.cursor..]
    }

    pub fn can_read(&self, length: usize) -> bool {
        self.cursor + length <= self.string.len()
    }

    pub fn can_read_char(&self) -> bool {
        self.can_read(1)
    }

    pub fn peek(&self, offset: usize) -> Result<char, BrigadierError> {
        if self.can_read(offset + 1) {
            Ok(self.string.chars().nth(self.cursor + offset).unwrap())
        } else {
            Err(BrigadierError::EOF)
        }
    }

    pub fn peek_char(&self) -> Result<char, BrigadierError> {
        self.peek(0)
    }

    pub fn read(&mut self) -> Result<char, BrigadierError> {
        if self.can_read_char() {
            let c = self.string.chars().nth(self.cursor).unwrap();
            self.cursor += c.len_utf8();
            Ok(c)
        } else {
            Err(BrigadierError::EOF)
        }
    }

    pub fn skip(&mut self) {
        self.cursor += 1;
    }

    pub fn is_allowed_number(&self, c: char) -> bool {
        c.is_numeric() || c == '-' || c == '.'
    }

    pub fn is_quoted_string_start(&self, c: char) -> bool {
        c == SYNTAX_DOUBLE_QUOTE || c == SYNTAX_SINGLE_QUOTE
    }

    pub fn skip_whitespace(&mut self) {
        while self.can_read_char() {
            let c = self.peek_char().unwrap();
            if !c.is_whitespace() {
                break;
            }
            self.skip();
        }
    }

    pub fn read_int(&mut self) -> Result<i32, BrigadierError> {
        let start = self.cursor;

        while self.can_read_char() && self.is_allowed_number(self.peek_char()?) {
            self.skip();
        }

        let number_str = &self.string[start..self.cursor];
        if number_str.is_empty() {
            Err(BrigadierError::ExpectedInt)
        } else {
            match number_str.parse::<i32>() {
                Ok(number) => Ok(number),
                Err(_) => {
                    self.cursor = start;
                    Err(BrigadierError::InvalidInt)
                }
            }
        }
    }

    pub fn read_long(&mut self) -> Result<i64, BrigadierError> {
        let start = self.cursor;

        while self.can_read_char() && self.is_allowed_number(self.peek_char()?) {
            self.skip();
        }

        let number_str = &self.string[start..self.cursor];
        if number_str.is_empty() {
            Err(BrigadierError::ExpectedLong)
        } else {
            match number_str.parse::<i64>() {
                Ok(number) => Ok(number),
                Err(_) => {
                    self.cursor = start;
                    Err(BrigadierError::InvalidLong)
                }
            }
        }
    }

    pub fn read_double(&mut self) -> Result<f64, BrigadierError> {
        let start = self.cursor;

        while self.can_read_char() && self.is_allowed_number(self.peek_char()?) {
            self.skip();
        }

        let number_str = &self.string[start..self.cursor];
        if number_str.is_empty() {
            Err(BrigadierError::ExpectedDouble)
        } else {
            match number_str.parse::<f64>() {
                Ok(number) => Ok(number),
                Err(_) => {
                    self.cursor = start;
                    Err(BrigadierError::InvalidDouble)
                }
            }
        }
    }

    pub fn read_float(&mut self) -> Result<f32, BrigadierError> {
        let start = self.cursor;

        while self.can_read_char() && self.is_allowed_number(self.peek_char()?) {
            self.skip();
        }

        let number_str = &self.string[start..self.cursor];
        if number_str.is_empty() {
            Err(BrigadierError::ExpectedFloat)
        } else {
            match number_str.parse::<f32>() {
                Ok(number) => Ok(number),
                Err(_) => {
                    self.cursor = start;
                    Err(BrigadierError::InvalidFloat)
                }
            }
        }
    }

    pub fn is_allowed_in_unquoted_string(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == '+'
    }

    pub fn read_unqoted_string(&mut self) -> Result<String, BrigadierError> {
        let start = self.cursor;

        while self.can_read_char() && self.is_allowed_in_unquoted_string(self.peek_char()?) {
            self.skip();
        }

        let string = &self.string[start..self.cursor];
        Ok(string.to_string())
    }

    pub fn read_quoted_string(&mut self) -> Result<String, BrigadierError> {
        if !self.can_read_char() {
            return Ok(String::new());
        }

        let next = self.peek_char()?;
        if !self.is_quoted_string_start(next) {
            return Err(BrigadierError::ExpectedStartOfQuote);
        }

        self.skip();

        return self.read_string_until(next);
    }

    pub fn read_string_until(&mut self, terminator: char) -> Result<String, BrigadierError> {
        let mut result = String::new();
        let mut escaped = false;

        while self.can_read_char() {
            let c = self.read()?;
            if escaped {
                if c == terminator || c == SYNTAX_ESCAPE {
                    result.push(c);
                    escaped = false;
                } else {
                    self.set_cursor(self.get_cursor() - 1);
                    return Err(BrigadierError::InvalidEscape(c));
                }
            } else if c == SYNTAX_ESCAPE {
                escaped = true;
            } else if c == terminator {
                return Ok(result);
            } else {
                result.push(c);
            }
        }

        Err(BrigadierError::ExpectedEndOfQuote)
    }

    pub fn read_string(&mut self) -> Result<String, BrigadierError> {
        if !self.can_read_char() {
            return Ok(String::new());
        }

        let next = self.peek_char()?;
        if self.is_quoted_string_start(next) {
            self.skip();
            return self.read_string_until(next);
        }

        self.read_unqoted_string()
    }

    pub fn read_boolean(&mut self) -> Result<bool, BrigadierError> {
        let start = self.cursor;
        let value: String = self.read_string()?;
        if value.is_empty() {
            Err(BrigadierError::ExpectedBool)
        } else {
            match value.as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => {
                    self.cursor = start;
                    Err(BrigadierError::InvalidBool)
                }
            }
        }
    }

    pub fn expect(&mut self, c: char) -> Result<(), BrigadierError> {
        if !self.can_read_char() {
            return Err(BrigadierError::ExpectedSymbol(c));
        }

        let next = self.peek_char()?;
        if next != c {
            return Err(BrigadierError::ExpectedSymbol(c));
        }

        self.skip();
        Ok(())
    }
}

impl<'a> Into<StringReader<'a>> for &'a str {
    fn into(self) -> StringReader<'a> {
        StringReader::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_read() {
        let mut reader = StringReader::new("abc");
        assert_eq!(reader.can_read_char(), true);
        reader.skip(); // a
        assert_eq!(reader.can_read_char(), true);
        reader.skip(); // b
        assert_eq!(reader.can_read_char(), true);
        reader.skip(); // c
        assert_eq!(reader.can_read_char(), false);
    }

    #[test]
    fn get_remaining_length() {
        let mut reader = StringReader::new("abc");
        assert_eq!(reader.get_remaining_length(), 3);
        reader.set_cursor(1);
        assert_eq!(reader.get_remaining_length(), 2);
        reader.set_cursor(2);
        assert_eq!(reader.get_remaining_length(), 1);
        reader.set_cursor(3);
        assert_eq!(reader.get_remaining_length(), 0);
    }

    #[test]
    fn can_read_length() {
        let reader = StringReader::new("abc");
        assert_eq!(reader.can_read(1), true);
        assert_eq!(reader.can_read(2), true);
        assert_eq!(reader.can_read(3), true);
        assert_eq!(reader.can_read(4), false);
        assert_eq!(reader.can_read(5), false);
    }

    #[test]
    fn peek() {
        let mut reader = StringReader::new("abc");
        assert_eq!(reader.peek_char().unwrap(), 'a');
        assert_eq!(reader.get_cursor(), 0);
        reader.set_cursor(2);
        assert_eq!(reader.peek_char().unwrap(), 'c');
        assert_eq!(reader.get_cursor(), 2);
    }

    #[test]
    fn peek_length() {
        let mut reader = StringReader::new("abc");
        assert_eq!(reader.peek(0).unwrap(), 'a');
        assert_eq!(reader.peek(2).unwrap(), 'c');
        assert_eq!(reader.get_cursor(), 0);
        reader.set_cursor(1);
        assert_eq!(reader.peek(1).unwrap(), 'c');
        assert_eq!(reader.get_cursor(), 1);
    }

    #[test]
    fn read() {
        let mut reader = StringReader::new("abc");
        assert_eq!(reader.read().unwrap(), 'a');
        assert_eq!(reader.read().unwrap(), 'b');
        assert_eq!(reader.read().unwrap(), 'c');
        assert_eq!(reader.get_cursor(), 3);
    }

    #[test]
    fn skip() {
        let mut reader = StringReader::new("abc");
        reader.skip();
        assert_eq!(reader.get_cursor(), 1);
        assert_eq!(reader.peek(0).unwrap(), 'b');
        assert_eq!(reader.read().unwrap(), 'b');
    }

    #[test]
    fn get_remaining() {
        let mut reader = StringReader::new("Hello!");
        assert_eq!(reader.get_remaining(), "Hello!");
        reader.set_cursor(3);
        assert_eq!(reader.get_remaining(), "lo!");
        reader.set_cursor(6);
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn get_read() {
        let mut reader = StringReader::new("Hello!");
        assert_eq!(reader.get_read(), "");
        reader.set_cursor(3);
        assert_eq!(reader.get_read(), "Hel");
        reader.set_cursor(6);
        assert_eq!(reader.get_read(), "Hello!");
    }

    #[test]
    fn skip_whitespace_none() {
        let mut reader = StringReader::new("Hello!");
        reader.skip_whitespace();
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn skip_whitespace_mixed() {
        let mut reader = StringReader::new(" \t \t\nHello!");
        reader.skip_whitespace();
        assert_eq!(reader.get_cursor(), 5);
    }

    #[test]
    fn skip_whitespace_empty() {
        let mut reader = StringReader::new("");
        reader.skip_whitespace();
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn read_unqoted_string() {
        let mut reader = StringReader::new("hello world");
        assert_eq!(reader.read_unqoted_string().unwrap(), "hello");
        assert_eq!(reader.get_cursor(), 5);
        assert_eq!(reader.get_read(), "hello");
        assert_eq!(reader.get_remaining(), " world");
    }

    #[test]
    fn read_unqoted_string_empty() {
        let mut reader = StringReader::new("");
        assert_eq!(reader.read_unqoted_string().unwrap(), "");
        assert_eq!(reader.get_cursor(), 0);
        assert_eq!(reader.get_read(), "");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_unqoted_string_empty_with_remaining() {
        let mut reader = StringReader::new(" hello world");
        assert_eq!(reader.read_unqoted_string().unwrap(), "");
        assert_eq!(reader.get_read(), "");
        assert_eq!(reader.get_remaining(), " hello world");
    }

    #[test]
    fn read_quoted_string() {
        let mut reader = StringReader::new("\"hello world\"");
        assert_eq!(reader.read_quoted_string().unwrap(), "hello world");
        assert_eq!(reader.get_cursor(), 13);
        assert_eq!(reader.get_read(), "\"hello world\"");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_single_quoted_string() {
        let mut reader = StringReader::new("'hello world'");
        assert_eq!(reader.read_quoted_string().unwrap(), "hello world");
        assert_eq!(reader.get_cursor(), 13);
        assert_eq!(reader.get_read(), "'hello world'");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_mixed_quoted_string_double_inside() {
        let mut reader = StringReader::new("'hello \"world\"'");
        assert_eq!(reader.read_quoted_string().unwrap(), "hello \"world\"");
        assert_eq!(reader.get_cursor(), 15);
        assert_eq!(reader.get_read(), "'hello \"world\"'");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_mixed_quoted_string_single_inside() {
        let mut reader = StringReader::new("\"hello 'world'\"");
        assert_eq!(reader.read_quoted_string().unwrap(), "hello 'world'");
        assert_eq!(reader.get_cursor(), 15);
        assert_eq!(reader.get_read(), "\"hello 'world'\"");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_quoted_string_empty() {
        let mut reader = StringReader::new("");
        assert_eq!(reader.read_quoted_string().unwrap(), "");
        assert_eq!(reader.get_cursor(), 0);
        assert_eq!(reader.get_read(), "");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_quoted_string_empty_quoted() {
        let mut reader = StringReader::new("\"\"");
        assert_eq!(reader.read_quoted_string().unwrap(), "");
        assert_eq!(reader.get_cursor(), 2);
        assert_eq!(reader.get_read(), "\"\"");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_quoted_string_empty_quoted_with_remaining() {
        let mut reader = StringReader::new("\"\" hello world");
        assert_eq!(reader.read_quoted_string().unwrap(), "");
        assert_eq!(reader.get_cursor(), 2);
        assert_eq!(reader.get_read(), "\"\"");
        assert_eq!(reader.get_remaining(), " hello world");
    }

    #[test]
    fn read_quoted_string_with_escaped_quote() {
        let mut reader = StringReader::new("\"hello \\\"world\\\"\"");
        assert_eq!(reader.read_quoted_string().unwrap(), "hello \"world\"");
        assert_eq!(reader.get_cursor(), 17);
        assert_eq!(reader.get_read(), "\"hello \\\"world\\\"\"");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_quoted_string_with_scaped_escapes() {
        let mut reader = StringReader::new("\"\\\\o/\"");
        assert_eq!(reader.read_quoted_string().unwrap(), "\\o/");
        assert_eq!(reader.get_cursor(), 6);
        assert_eq!(reader.get_read(), "\"\\\\o/\"");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_quoted_string_with_remaining() {
        let mut reader = StringReader::new("\"hello world\" foo bar");
        assert_eq!(reader.read_quoted_string().unwrap(), "hello world");
        assert_eq!(reader.get_cursor(), 13);
        assert_eq!(reader.get_read(), "\"hello world\"");
        assert_eq!(reader.get_remaining(), " foo bar");
    }

    #[test]
    fn read_quoted_string_with_immediate_remaining() {
        let mut reader = StringReader::new("\"hello world\"foo bar");
        assert_eq!(reader.read_quoted_string().unwrap(), "hello world");
        assert_eq!(reader.get_cursor(), 13);
        assert_eq!(reader.get_read(), "\"hello world\"");
        assert_eq!(reader.get_remaining(), "foo bar");
    }

    #[test]
    fn read_quoted_string_no_open() {
        let mut reader = StringReader::new("hello world\"");
        assert_eq!(
            reader.read_quoted_string(),
            Err(BrigadierError::ExpectedStartOfQuote)
        );
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn read_quoted_string_no_close() {
        let mut reader = StringReader::new("\"hello world");
        assert_eq!(
            reader.read_quoted_string(),
            Err(BrigadierError::ExpectedEndOfQuote)
        );
        assert_eq!(reader.get_cursor(), 12);
    }

    #[test]
    fn read_quoted_string_invalid_escape() {
        let mut reader = StringReader::new("\"hello\\nworld\"");
        assert_eq!(
            reader.read_quoted_string(),
            Err(BrigadierError::InvalidEscape('n'))
        );
        assert_eq!(reader.get_cursor(), 7);
    }

    #[test]
    fn read_quoted_string_invalid_quote_escape() {
        let mut reader = StringReader::new("'hello\\\"\'world");
        assert_eq!(
            reader.read_quoted_string(),
            Err(BrigadierError::InvalidEscape('"'))
        );
        assert_eq!(reader.get_cursor(), 7);
    }

    #[test]
    fn read_string_no_quotes() {
        let mut reader = StringReader::new("hello world");
        assert_eq!(reader.read_string().unwrap(), "hello");
        assert_eq!(reader.get_cursor(), 5);
        assert_eq!(reader.get_read(), "hello");
        assert_eq!(reader.get_remaining(), " world");
    }

    #[test]
    fn read_string_single_quotes() {
        let mut reader = StringReader::new("'hello world'");
        assert_eq!(reader.read_string().unwrap(), "hello world");
        assert_eq!(reader.get_cursor(), 13);
        assert_eq!(reader.get_read(), "'hello world'");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_string_double_quotes() {
        let mut reader = StringReader::new("\"hello world\"");
        assert_eq!(reader.read_string().unwrap(), "hello world");
        assert_eq!(reader.get_cursor(), 13);
        assert_eq!(reader.get_read(), "\"hello world\"");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_int() {
        let mut reader = StringReader::new("1234567890");
        assert_eq!(reader.read_int().unwrap(), 1234567890);
        assert_eq!(reader.get_cursor(), 10);
        assert_eq!(reader.get_read(), "1234567890");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_int_negative() {
        let mut reader = StringReader::new("-1234567890");
        assert_eq!(reader.read_int().unwrap(), -1234567890);
        assert_eq!(reader.get_cursor(), 11);
        assert_eq!(reader.get_read(), "-1234567890");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_int_invalid() {
        let mut reader = StringReader::new("12.34");
        assert_eq!(reader.read_int(), Err(BrigadierError::InvalidInt));
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn read_int_none() {
        let mut reader = StringReader::new("");
        assert_eq!(reader.read_int(), Err(BrigadierError::ExpectedInt));
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn read_int_with_remaining() {
        let mut reader = StringReader::new("1234567890 foo bar");
        assert_eq!(reader.read_int().unwrap(), 1234567890);
        assert_eq!(reader.get_cursor(), 10);
        assert_eq!(reader.get_read(), "1234567890");
        assert_eq!(reader.get_remaining(), " foo bar");
    }

    #[test]
    fn read_int_with_remaining_immediate() {
        let mut reader = StringReader::new("1234567890foo bar");
        assert_eq!(reader.read_int().unwrap(), 1234567890);
        assert_eq!(reader.get_cursor(), 10);
        assert_eq!(reader.get_read(), "1234567890");
        assert_eq!(reader.get_remaining(), "foo bar");
    }

    #[test]
    fn read_long() {
        let mut reader = StringReader::new("1234567890");
        assert_eq!(reader.read_long().unwrap(), 1234567890i64);
        assert_eq!(reader.get_cursor(), 10);
        assert_eq!(reader.get_read(), "1234567890");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_long_negative() {
        let mut reader = StringReader::new("-1234567890");
        assert_eq!(reader.read_long().unwrap(), -1234567890i64);
        assert_eq!(reader.get_cursor(), 11);
        assert_eq!(reader.get_read(), "-1234567890");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_long_invalid() {
        let mut reader = StringReader::new("12.34");
        assert_eq!(reader.read_long(), Err(BrigadierError::InvalidLong));
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn read_long_none() {
        let mut reader = StringReader::new("");
        assert_eq!(reader.read_long(), Err(BrigadierError::ExpectedLong));
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn read_long_with_remaining() {
        let mut reader = StringReader::new("1234567890 foo bar");
        assert_eq!(reader.read_long().unwrap(), 1234567890i64);
        assert_eq!(reader.get_cursor(), 10);
        assert_eq!(reader.get_read(), "1234567890");
        assert_eq!(reader.get_remaining(), " foo bar");
    }

    #[test]
    fn read_long_with_remaining_immediate() {
        let mut reader = StringReader::new("1234567890foo bar");
        assert_eq!(reader.read_long().unwrap(), 1234567890i64);
        assert_eq!(reader.get_cursor(), 10);
        assert_eq!(reader.get_read(), "1234567890");
        assert_eq!(reader.get_remaining(), "foo bar");
    }

    #[test]
    fn read_double() {
        let mut reader = StringReader::new("123");
        assert_eq!(reader.read_double().unwrap(), 123.0f64);
        assert_eq!(reader.get_cursor(), 3);
        assert_eq!(reader.get_read(), "123");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_double_with_decimal() {
        let mut reader = StringReader::new("12.34");
        assert_eq!(reader.read_double().unwrap(), 12.34f64);
        assert_eq!(reader.get_cursor(), 5);
        assert_eq!(reader.get_read(), "12.34");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_double_negative() {
        let mut reader = StringReader::new("-12.34");
        assert_eq!(reader.read_double().unwrap(), -12.34f64);
        assert_eq!(reader.get_cursor(), 6);
        assert_eq!(reader.get_read(), "-12.34");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_double_invalid() {
        let mut reader = StringReader::new("12.34.56");
        assert_eq!(reader.read_double(), Err(BrigadierError::InvalidDouble));
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn read_double_none() {
        let mut reader = StringReader::new("");
        assert_eq!(reader.read_double(), Err(BrigadierError::ExpectedDouble));
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn read_double_with_remaining() {
        let mut reader = StringReader::new("12.34 foo bar");
        assert_eq!(reader.read_double().unwrap(), 12.34f64);
        assert_eq!(reader.get_cursor(), 5);
        assert_eq!(reader.get_read(), "12.34");
        assert_eq!(reader.get_remaining(), " foo bar");
    }

    #[test]
    fn read_double_with_remaining_immediate() {
        let mut reader = StringReader::new("12.34foo bar");
        assert_eq!(reader.read_double().unwrap(), 12.34f64);
        assert_eq!(reader.get_cursor(), 5);
        assert_eq!(reader.get_read(), "12.34");
        assert_eq!(reader.get_remaining(), "foo bar");
    }

    #[test]
    fn read_float() {
        let mut reader = StringReader::new("123");
        assert_eq!(reader.read_float().unwrap(), 123.0f32);
        assert_eq!(reader.get_cursor(), 3);
        assert_eq!(reader.get_read(), "123");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_float_with_decimal() {
        let mut reader = StringReader::new("12.34");
        assert_eq!(reader.read_float().unwrap(), 12.34f32);
        assert_eq!(reader.get_cursor(), 5);
        assert_eq!(reader.get_read(), "12.34");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_float_negative() {
        let mut reader = StringReader::new("-123");
        assert_eq!(reader.read_float().unwrap(), -123.0f32);
        assert_eq!(reader.get_cursor(), 4);
        assert_eq!(reader.get_read(), "-123");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_float_invalid() {
        let mut reader = StringReader::new("12.34.56");
        assert_eq!(reader.read_float(), Err(BrigadierError::InvalidFloat));
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn read_float_none() {
        let mut reader = StringReader::new("");
        assert_eq!(reader.read_float(), Err(BrigadierError::ExpectedFloat));
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn read_float_with_remaining() {
        let mut reader = StringReader::new("12.34 foo bar");
        assert_eq!(reader.read_float().unwrap(), 12.34f32);
        assert_eq!(reader.get_cursor(), 5);
        assert_eq!(reader.get_read(), "12.34");
        assert_eq!(reader.get_remaining(), " foo bar");
    }

    #[test]
    fn read_float_with_remaining_immediate() {
        let mut reader = StringReader::new("12.34foo bar");
        assert_eq!(reader.read_float().unwrap(), 12.34f32);
        assert_eq!(reader.get_cursor(), 5);
        assert_eq!(reader.get_read(), "12.34");
        assert_eq!(reader.get_remaining(), "foo bar");
    }

    #[test]
    fn expect_correct() {
        let mut reader = StringReader::new("abc");
        reader.expect('a').unwrap();
        assert_eq!(reader.get_cursor(), 1);
    }

    #[test]
    fn expect_incorrect() {
        let mut reader = StringReader::new("bca");
        assert_eq!(reader.expect('a'), Err(BrigadierError::ExpectedSymbol('a')));
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn expect_none() {
        let mut reader = StringReader::new("");
        assert_eq!(reader.expect('a'), Err(BrigadierError::ExpectedSymbol('a')));
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn read_boolean_correct() {
        let mut reader = StringReader::new("true");
        assert_eq!(reader.read_boolean().unwrap(), true);
        assert_eq!(reader.get_cursor(), 4);
        assert_eq!(reader.get_read(), "true");
        assert_eq!(reader.get_remaining(), "");
    }

    #[test]
    fn read_boolean_incorrect() {
        let mut reader = StringReader::new("tuesday");
        assert_eq!(reader.read_boolean(), Err(BrigadierError::InvalidBool));
        assert_eq!(reader.get_cursor(), 0);
    }

    #[test]
    fn read_boolean_none() {
        let mut reader = StringReader::new("");
        assert_eq!(reader.read_boolean(), Err(BrigadierError::ExpectedBool));
        assert_eq!(reader.get_cursor(), 0);
    }
}
