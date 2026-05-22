use std::io;

#[derive(Debug)]
pub enum KwpError {
    Message(String),
    Io(io::Error),
}

impl From<io::Error> for KwpError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl std::fmt::Display for KwpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KwpError::Message(msg) => f.write_str(msg),
            KwpError::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for KwpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            KwpError::Message(_) => None,
            KwpError::Io(err) => Some(err),
        }
    }
}

pub type Result<T> = std::result::Result<T, KwpError>;
