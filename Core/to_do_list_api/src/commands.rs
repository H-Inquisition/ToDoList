use axum::extract::{Json, State};
use core::common::Task;
use core::app_state::AppState;
use std::sync::{Arc, Mutex};

pub async fn create_task_command(State(app_state): State<Arc<Mutex<AppState>>>, Json(task): Json<Task>) -> String {
    let mut state = app_state.lock().unwrap();
    state.add_task(task);
    state.get_tasks()
}

pub async fn update_task_command(State(app_state): State<Arc<Mutex<AppState>>>, Json((id, task)): Json<(u32, Task)>) -> String {
    let mut state = app_state.lock().unwrap();
    state.update_task(id, task);
    state.get_tasks()
}

pub async fn delete_task_command(State(app_state): State<Arc<Mutex<AppState>>>, Json(id): Json<u32>) -> String {
    let mut state = app_state.lock().unwrap();
    state.delete_task(id);
    state.get_tasks()
}