use crate::error::{Error, Result};
use axum::Router;
use axum::routing::{get, post};
use core::app_state::AppState;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

pub async fn start_server(address: &str) -> Result<()> {
    let state = Arc::new(Mutex::new(AppState::new(address)?));

    let router = Router::new()
        .route("/port", get(api::queries::port_query))
        .with_state(state.clone())
        .route("/list_of_tasks", get(api::queries::list_of_tasks_query))
        .with_state(state.clone())
        .route("/create_task", post(api::commands::create_task_command))
        .with_state(state.clone())
        .route(
            "/update_task_status",
            post(api::commands::update_task_status_command),
        )
        .with_state(state.clone())
        .route(
            "/update_task_title",
            post(api::commands::update_task_title_command),
        )
        .with_state(state.clone())
        .route(
            "/update_task_priority",
            post(api::commands::update_task_priority_command),
        )
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
