#[tokio::main]
async fn main() {
    let address_result = runner::handlers::get_port();
    let port = runner::handlers::handle_user_input_result(address_result).expect("Handling an error should not fail.");
    let result = runner::handlers::start_server(format!("0.0.0.0:{}", port).as_str(), "task_database.db").await;
    runner::handlers::handle_result(result).expect("Handling an error should not fail.");
}
