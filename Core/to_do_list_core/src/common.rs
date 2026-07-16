use rusqlite::ToSql;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Task {
    pub status: Status,
    pub title: String,
    pub priority: Priority,
}

#[derive(Debug, Clone, PartialEq, strum_macros::Display, Deserialize, Serialize)]
pub enum Status {
    Planned,
    Done,
}

impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "Planned" => Ok(Self::Planned),
            "Done" => Ok(Self::Done),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Status {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(match self {
            Self::Planned => "Planned",
            Self::Done => "Done",
        }
        .into())
    }
}

#[derive(Debug, Clone, PartialEq, strum_macros::Display, Deserialize, Serialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl FromSql for Priority {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "Low" => Ok(Self::Low),
            "Medium" => Ok(Self::Medium),
            "High" => Ok(Self::High),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Priority {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
        }
        .into())
    }
}
