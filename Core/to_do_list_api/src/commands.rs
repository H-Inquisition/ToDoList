use crate::error::*;
use axum::extract::{Json, State};
use core::app_state::AppState;
use core::common::{Priority, Status, Task};
use std::sync::{Arc, Mutex};

// Command is used to create a new task instance in the database
//
// Arguments required to be provided:
//   task_title: String - a name of the task, can be a task description
//   priority: String - has to be convertable to the Priority enum, see enum values
//
// Returns:
//   list_of_tasks: Vec<(i64, Task)> - a list of tuples consisting of the tasks along with their ids
//
// Call:
//   curl -X POST http://localhost:insert_port_number/create_task \
//     -H "Content-Type: application/json" \
//     -d '[task_title, priority]'
//
// Example:
//   curl -X POST http://localhost:3000/create_task \
//     -H "Content-Type: application/json" \
//     -d '["TaskName", "High"]'
pub async fn create_task_command(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json((title, priority)): Json<(String, Priority)>,
) -> Result<Json<Vec<(i64, Task)>>> {
    let mut state = app_state.lock().map_err(|_| {
        Error::MutexLockFailed("Failed to lock the AppState while creating a task.")
    })?;

    state.add_task(title, priority)?;
    Ok(Json(state.get_tasks()?))
}

// Command is used to update a preexisting task in the database
//
// Arguments required to be provided:
//   task_id: i64 - an assigned id of the task in the database
//   task_status: String - has to be convertable to the Status enum, see Status enum values
//   task_title: String - a new name of the task
//   priority: String - has to be convertable to the Priority enum, see Priority enum values
//
// Returns:
//   list_of_tasks: Vec<(i64, Task)> - a list of tuples consisting of the tasks along with their ids
//
// Call:
//   curl -X POST http://localhost:insert_port_number/update_task \
//     -H "Content-Type: application/json" \
//     -d '[task_id, task_status, task_title, priority]'
//
// Example:
//   curl -X POST http://localhost:3000/update_task \
//     -H "Content-Type: application/json" \
//     -d '[1, "Done", "NewTaskName", "High"]'
pub async fn update_task_command(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json((id, status, title, priority)): Json<(i64, Status, String, Priority)>,
) -> Result<Json<Vec<(i64, Task)>>> {
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
    Ok(Json(state.get_tasks()?))
}

// Command is used to delete a task from a database
//
// Arguments required to be provided:
//   id: i64 - an assigned id of the task in the database
//
// Returns:
//   list_of_tasks: Vec<(i64, Task)> - a list of tuples consisting of the tasks along with their ids
//
// Call:
//   curl -X POST http://localhost:insert_port_number/delete_task \
//     -H "Content-Type: application/json" \
//     -d 'id'
//
// Example:
//   curl -X POST http://localhost:3000/delete_task \
//     -H "Content-Type: application/json" \
//     -d '2'
pub async fn delete_task_command(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(id): Json<i64>,
) -> Result<Json<Vec<(i64, Task)>>> {
    let mut state = app_state.lock().map_err(|_| {
        Error::MutexLockFailed("Failed to lock the AppState while deleting a task.")
    })?;
    state.delete_task(id)?;
    Ok(Json(state.get_tasks()?))
}
