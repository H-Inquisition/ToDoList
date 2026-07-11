use rusqlite::types::FromSqlError;

#[derive(Debug, Clone)]
pub enum Error {
    OpenDatabaseFailed,
    ExecuteDatabaseCommandFailed(String),
    PrepareDatabaseQueryFailed(String),
    ConversionToSqlFailed(String),
    ConversionFromSqlFailed(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Error::ConversionToSqlFailed(format!("{:?}", value))
    }
}

impl From<FromSqlError> for Error {
    fn from(value: FromSqlError) -> Self {
        Error::ConversionFromSqlFailed(format!("{:?}", value))
    }
}
