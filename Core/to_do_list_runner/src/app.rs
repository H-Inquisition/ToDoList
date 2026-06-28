use axum::Router;
use tokio::net::TcpListener;
use crate::error::{Error, Result};
use axum::routing::{get, post};
use std::sync::{Arc, Mutex};
use core::app_state::AppState;

pub async fn run(address: &str) -> Result<()> {
    let state = Arc::new(Mutex::new(AppState::new(address)));

    let router = Router::new()
        .route("/port", get(api::queries::port_query)).with_state(state.clone())
        .route("/list_of_tasks", get(api::queries::list_of_tasks_query)).with_state(state.clone())
        .route("/create_task", post(api::commands::create_task_command)).with_state(state.clone())
        .route("/update_task", post(api::commands::update_task_command)).with_state(state.clone())
        .route("/delete_task", post(api::commands::delete_task_command)).with_state(state.clone());

    let listener = TcpListener::bind(address).await.map_err(|_| Error::PortBinding(format!("Failed to bind the listener to the address: {}", address)))?;

    axum::serve(listener, router).await.map_err(|_| Error::Runner("An error has occurred during the server runtime."))
}