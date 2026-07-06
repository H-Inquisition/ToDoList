#[derive(Debug, Clone)]
pub enum Error {
    OpenDatabaseFailed,
    ExecuteDatabaseCommandFailed(String),
}

pub type Result<T> = std::result::Result<T, Error>;
