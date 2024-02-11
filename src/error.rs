use std::fmt::Debug;

pub enum E {
    IO(std::io::Error),
    InputEncodingError,
    LexingFailed {
        line: usize,
        column: usize,
    },
    LexingTriviaFailed,
    ParsingFailed {
        token: String,
        line: usize,
        column: usize,
    },
    ParsingTriviaFailed,
}

impl From<std::io::Error> for E {
    fn from(value: std::io::Error) -> Self {
        E::IO(value)
    }
}

impl From<std::str::Utf8Error> for E {
    fn from(_: std::str::Utf8Error) -> Self {
        E::InputEncodingError
    }
}

pub type R = Result<(), E>;

impl Debug for E {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            E::IO(e) => {
                f.write_fmt(format_args!("{e:?}"))?;
                Ok(())
            }
            E::InputEncodingError => {
                f.write_str("Incorrect input encoding - expected UTF-8")?;
                Ok(())
            }
            E::LexingFailed { line, column } => {
                f.write_fmt(format_args!("Unknown token (line={line}, column={column})"))?;
                Ok(())
            }
            E::LexingTriviaFailed => {
                f.write_str("Failed to lex trivia - this is likely a bug")?;
                Ok(())
            }
            E::ParsingFailed {
                token,
                line,
                column,
            } => {
                f.write_fmt(format_args!(
                    "Unexpected token \"{token}\" (line={line}, column={column})"
                ))?;
                Ok(())
            }
            E::ParsingTriviaFailed => {
                f.write_str("Failed to parse trivia - this is likely a bug")?;
                Ok(())
            }
        }
    }
}
