#[derive(Debug)]
pub enum E {
    IO(std::io::Error),
    LexingWhitespaceFailed,
    ParsingFailed,
}

impl From<std::io::Error> for E {
    fn from(value: std::io::Error) -> Self {
        E::IO(value)
    }
}

impl From<()> for E {
    fn from(_: ()) -> Self {
        E::LexingWhitespaceFailed
    }
}

impl<'a> From<peg::error::ParseError<usize>> for E {
    fn from(_: peg::error::ParseError<usize>) -> Self {
        E::ParsingFailed
    }
}

pub type R = Result<(), E>;
