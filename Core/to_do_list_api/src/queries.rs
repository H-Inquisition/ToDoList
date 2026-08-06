use crate::error::*;
use axum::extract::{Json, State};
use core::app_state::AppState;
use std::sync::{Arc, Mutex};
use core::common::Task;

// Query is used to get a list of tasks from the database
//
// Returns:
//   list_of_tasks: Vec<(i64, Task)> - a list of tuples consisting of the tasks along with their ids
//
// Call:
//   curl http://localhost:insert_port_number/list_of_tasks
pub async fn list_of_tasks_query(State(app_state): State<Arc<Mutex<AppState>>>) -> Result<Json<Vec<(i64, Task)>>> {
    Ok(Json(app_state
        .lock()
        .map_err(|_| {
            Error::MutexLockFailed("Failed to lock the AppState while querying the task list.")
        })?
        .get_tasks()?))
}
