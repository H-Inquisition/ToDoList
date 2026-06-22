use axum::Router;
use axum::routing::{get, post};
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(api::runner::root)).route("/task", post(api::receiver::create_task));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
