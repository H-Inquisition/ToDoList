use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub status: Status,
    pub title: String,
    pub priority: Priority,
}

impl Task {
    pub fn to_database(&self) {

    }
}

#[derive(Debug, Clone, strum_macros::Display, Deserialize, Serialize)]
pub enum Status {
    Planned,
    Done,
}

#[derive(Debug, Clone, strum_macros::Display, Deserialize, Serialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}