use crate::error::*;
use axum::extract::State;
use core::app_state::AppState;
use std::sync::{Arc, Mutex};

pub async fn port_query(State(app_state): State<Arc<Mutex<AppState>>>) -> Result<String> {
    Ok(app_state
        .lock()
        .map_err(|_| {
            Error::MutexLockFailed("Failed to lock the AppState while querying the port.")
        })?
        .get_port())
}

pub async fn list_of_tasks_query(State(app_state): State<Arc<Mutex<AppState>>>) -> Result<String> {
    Ok(app_state
        .lock()
        .map_err(|_| {
            Error::MutexLockFailed("Failed to lock the AppState while querying the task list.")
        })?
        .get_tasks_as_string()?)
}
