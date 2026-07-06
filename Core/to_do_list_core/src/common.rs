use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub status: Status,
    pub title: String,
    pub priority: Priority,
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
