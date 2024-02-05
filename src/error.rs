#[derive(Debug)]
pub enum E {
    IO(std::io::Error),
    LexingWhitespaceFailed,
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

pub type R = Result<(), E>;
