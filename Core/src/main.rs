
#[tokio::main]
async fn main() {
    let result = runner::app::run("0.0.0.0:3000").await;
    runner::app::handle_error(result).expect("Handling an error should not fail.");
}
