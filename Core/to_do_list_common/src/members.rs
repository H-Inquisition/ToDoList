use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Status {
    Planned,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}