use crate::error::{Error, Result};
use axum::Router;
use axum::routing::{get, post};
use core::app_state::AppState;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

pub async fn start_server(address: &str, database_path: &str) -> Result<()> {
    let state = Arc::new(Mutex::new(AppState::new(address, database_path)?));

    let router = Router::new()
        .route("/port", get(api::queries::port_query))
        .with_state(state.clone())
        .route("/list_of_tasks", get(api::queries::list_of_tasks_query))
        .with_state(state.clone())
        .route("/create_task", post(api::commands::create_task_command))
        .with_state(state.clone())
        .route("/update_task", post(api::commands::update_task_command))
        .with_state(state.clone())
        .route("/delete_task", post(api::commands::delete_task_command))
        .with_state(state.clone());

    let listener = TcpListener::bind(address).await.map_err(|_| {
        Error::PortBinding(format!(
            "Failed to bind the listener to the address: {}",
            address
        ))
    })?;

    axum::serve(listener, router)
        .await
        .map_err(|_| Error::Runner("An error has occurred during the server runtime."))
}

pub fn handle_error<T>(result: Result<T>) -> Result<()> {
    if let Some(err) = result.as_ref().err() {
        log_error(err)?;
    }
    Ok(())
}

fn log_error(err: &Error) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open("./error_log.txt")
        .map_err(|_| Error::OpenFile)?;
    writeln!(file, "{}", err).map_err(|_| Error::WriteFile)
}

#[cfg(test)]
pub mod tests {
    use core::app_state::AppState;
    use core::common::Priority;

    #[ignore]
    #[test]
    fn set_up_database_example() {
        let mut app_state =
            AppState::new("0.0.0.0:3000", "example_database.db").unwrap();

        app_state
            .add_task("test_task_one".to_string(), Priority::Medium)
            .unwrap();
        app_state
            .add_task("test_task_two".to_string(), Priority::High)
            .unwrap();
        app_state
            .add_task("test_task_three".to_string(), Priority::Medium)
            .unwrap();
        app_state
            .add_task("test_task_four".to_string(), Priority::Low)
            .unwrap();
        app_state
            .add_task("test_task_five".to_string(), Priority::Medium)
            .unwrap();
        app_state
            .add_task("test_task_six".to_string(), Priority::Medium)
            .unwrap();
        app_state
            .add_task("test_task_seven".to_string(), Priority::High)
            .unwrap();
        app_state
            .add_task("test_task_eight".to_string(), Priority::Medium)
            .unwrap();
        app_state
            .add_task("test_task_nine".to_string(), Priority::Low)
            .unwrap();
        app_state
            .add_task("test_task_ten".to_string(), Priority::Medium)
            .unwrap();
        app_state
            .add_task("test_task_eleven".to_string(), Priority::Medium)
            .unwrap();
        app_state
            .add_task("test_task_twelve".to_string(), Priority::High)
            .unwrap();
        app_state
            .add_task("test_task_thirteen".to_string(), Priority::Medium)
            .unwrap();
        app_state
            .add_task("test_task_fourteen".to_string(), Priority::Low)
            .unwrap();
        app_state
            .add_task("test_task_fifteen".to_string(), Priority::Medium)
            .unwrap();
        app_state
            .add_task("test_task_sixteen".to_string(), Priority::Medium)
            .unwrap();
        app_state
            .add_task("test_task_seventeen".to_string(), Priority::High)
            .unwrap();
        app_state
            .add_task("test_task_eighteen".to_string(), Priority::Medium)
            .unwrap();
        app_state
            .add_task("test_task_nineteen".to_string(), Priority::Low)
            .unwrap();
        app_state
            .add_task("test_task_twenty".to_string(), Priority::Medium)
            .unwrap();
    }
}
