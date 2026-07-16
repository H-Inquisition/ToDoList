#[tokio::main]
async fn main() {
    let result = runner::handlers::start_server("0.0.0.0:3000", "task_database.db").await;
    runner::handlers::handle_error(result).expect("Handling an error should not fail.");
}
