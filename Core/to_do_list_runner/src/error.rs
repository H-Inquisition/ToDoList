#[derive(Debug, Clone)]
pub enum Error {
    PortBinding(String),
    Runner(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;