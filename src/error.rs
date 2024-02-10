#[derive(Debug)]
pub enum E {
    IO(std::io::Error),
    InputEncodingError,
    LexingFailed { line: usize, column: usize },
    LexingTriviaFailed,
    ParsingFailed,
}

impl From<std::io::Error> for E {
    fn from(value: std::io::Error) -> Self {
        E::IO(value)
    }
}

impl From<peg::error::ParseError<usize>> for E {
    fn from(_: peg::error::ParseError<usize>) -> Self {
        E::ParsingFailed
    }
}

impl From<std::str::Utf8Error> for E {
    fn from(_: std::str::Utf8Error) -> Self {
        E::InputEncodingError
    }
}

pub type R = Result<(), E>;
