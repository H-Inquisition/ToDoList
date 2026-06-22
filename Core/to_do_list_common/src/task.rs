use crate::members::{Priority, Status};
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