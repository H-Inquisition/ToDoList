use crate::error::*;
use axum::extract::{Json, State};
use core::app_state::AppState;
use core::common::{Priority, Status, Task};
use std::sync::{Arc, Mutex};

pub async fn create_task_command(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json((title, priority)): Json<(String, Priority)>,
) -> Result<String> {
    let mut state = app_state.lock().map_err(|_| {
        Error::MutexLockFailed("Failed to lock the AppState while creating a task.")
    })?;

    state.add_task(title, priority)?;
    Ok(state.get_tasks()?)
}

pub async fn update_task_command(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json((id, status, title, priority)): Json<(i64, Status, String, Priority)>,
) -> Result<String> {
    let mut state = app_state.lock().map_err(|_| {
        Error::MutexLockFailed("Failed to lock the AppState while updating a task status.")
    })?;
    state.update_task(
        id,
        Task {
            status,
            title,
            priority,
        },
    )?;
    Ok(state.get_tasks()?)
}

pub async fn delete_task_command(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(id): Json<i64>,
) -> Result<String> {
    let mut state = app_state.lock().map_err(|_| {
        Error::MutexLockFailed("Failed to lock the AppState while deleting a task.")
    })?;
    state.delete_task(id)?;
    Ok(state.get_tasks()?)
}
