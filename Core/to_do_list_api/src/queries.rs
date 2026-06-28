use core::app_state::AppState;
use axum::extract::State;
use std::sync::{Arc, Mutex};

pub async fn port_query(State(app_state): State<Arc<Mutex<AppState>>>) -> String {
    app_state.lock().unwrap().get_port()
}

pub async fn list_of_tasks_query(State(app_state): State<Arc<Mutex<AppState>>>) -> String {
    app_state.lock().unwrap().get_tasks()
}