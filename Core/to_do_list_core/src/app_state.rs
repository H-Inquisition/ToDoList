use crate::common::{Priority, Status, Task};
use crate::database_handler::DatabaseHandler;
use crate::error::*;

pub struct AppState {
    port: String,
    database_connection: DatabaseHandler,
}

impl AppState {
    pub fn new(address: &str, database_path: &str) -> Result<Self> {
        Ok(Self {
            port: address.to_string(),
            database_connection: DatabaseHandler::new(database_path)?,
        })
    }

    pub fn get_port(&self) -> String {
        format!("Currently running on port: {}\n", self.port)
    }

    pub fn get_tasks_as_vector(&self) -> Result<Vec<(i64, Task)>> {
        self.database_connection.get_tasks_as_vector()
    }

    pub fn get_tasks_as_string(&self) -> Result<String> {
        self.database_connection.get_tasks_as_string()
    }

    pub fn add_task(&mut self, title: String, priority: Priority) -> Result<()> {
        self.database_connection.add_task(Task {
            status: Status::Planned,
            title,
            priority,
        })
    }

    pub fn update_task(&mut self, id: i64, task: Task) -> Result<()> {
        self.database_connection.update_task(id, task)
    }

    pub fn delete_task(&mut self, id: i64) -> Result<()> {
        self.database_connection.remove_task(id)
    }
}
