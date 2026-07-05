use axum::extract::{Json, State};
use core::common::{Status, Priority};
use core::app_state::AppState;
use std::sync::{Arc, Mutex};
use crate::error::*;

pub async fn create_task_command(State(app_state): State<Arc<Mutex<AppState>>>, Json((title, priority)): Json<(String, Priority)>) -> Result<String> {
    let mut state = app_state.lock().map_err(|_| Error::MutexLockFailed("Failed to lock the AppState while creating a task."))?;
    state.add_task(title, priority);
    Ok(state.get_tasks())
}

pub async fn update_task_status_command(State(app_state): State<Arc<Mutex<AppState>>>, Json((id, status)): Json<(u32, Status)>) -> Result<String> {
    let mut state = app_state.lock().map_err(|_| Error::MutexLockFailed("Failed to lock the AppState while updating a task status."))?;
    state.update_task_status(id, status);
    Ok(state.get_tasks())
}

pub async fn update_task_title_command(State(app_state): State<Arc<Mutex<AppState>>>, Json((id, title)): Json<(u32, String)>) -> Result<String> {
    let mut state = app_state.lock().map_err(|_| Error::MutexLockFailed("Failed to lock the AppState while updating a task title."))?;
    state.update_task_title(id, title);
    Ok(state.get_tasks())
}
pub async fn update_task_priority_command(State(app_state): State<Arc<Mutex<AppState>>>, Json((id, priority)): Json<(u32, Priority)>) -> Result<String> {
    let mut state = app_state.lock().map_err(|_| Error::MutexLockFailed("Failed to lock the AppState while updating a task priority."))?;
    state.update_task_priority(id, priority);
    Ok(state.get_tasks())
}

pub async fn delete_task_command(State(app_state): State<Arc<Mutex<AppState>>>, Json(id): Json<u32>) -> Result<String> {
    let mut state = app_state.lock().map_err(|_| Error::MutexLockFailed("Failed to lock the AppState while deleting a task."))?;
    state.delete_task(id);
    Ok(state.get_tasks())
}