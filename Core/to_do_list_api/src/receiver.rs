use axum::extract::Json as RequestJson;
use axum::response::Json as ResponseJson;
use common::task::Task;

pub async fn create_task(RequestJson(task): RequestJson<Task>) -> ResponseJson<Task> {
    ResponseJson(task)
}