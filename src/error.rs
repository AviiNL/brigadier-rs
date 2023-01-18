#[derive(Debug, PartialEq)]
pub enum BrigadierError {
    EOF,
    ExpectedSymbol(char),
    ExpectedInt,
    ExpectedLong,
    ExpectedDouble,
    ExpectedFloat,
    ExpectedBool,
    ExpectedUnquotedString,
    ExpectedStartOfQuote,
    ExpectedEndOfQuote,

    InvalidInt,
    InvalidLong,
    InvalidDouble,
    InvalidFloat,
    InvalidEscape(char),
    InvalidBool,

    IntegerTooLow { result: i32, min: i32 },
    IntegerTooHigh { result: i32, max: i32 },
    DoubleTooLow { result: f64, min: f64 },
    DoubleTooHigh { result: f64, max: f64 },
    FloatTooLow { result: f32, min: f32 },
    FloatTooHigh { result: f32, max: f32 },
    LongTooLow { result: i64, min: i64 },
    LongTooHigh { result: i64, max: i64 },
}
