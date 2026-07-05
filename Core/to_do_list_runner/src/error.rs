use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Error {
    PortBinding(String),
    Runner(&'static str),
    OpenFile,
    WriteFile,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PortBinding(message) => write!(f, "Port Binding Error: {}", message),
            Error::Runner(message) => write!(f, "Runner Error: {}", message),
            Error::OpenFile => write!(f, "Failed to open a file."),
            Error::WriteFile => write!(f, "Failed to write to a file."),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;