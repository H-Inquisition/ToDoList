
#[tokio::main]
async fn main() {
    runner::app::run("0.0.0.0:3000").await.unwrap();
}
