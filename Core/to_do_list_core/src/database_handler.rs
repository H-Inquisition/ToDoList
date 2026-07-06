use crate::common::Task;
use crate::error::*;
use rusqlite::Connection;

pub struct DatabaseHandler {
    connection: Connection,
}

impl DatabaseHandler {
    pub fn new() -> Result<Self> {
        let connection =
            Connection::open("task_database.db").map_err(|_| Error::OpenDatabaseFailed)?;
        connection.execute("CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, status TEXT NOT NULL, title TEXT, priority TEXT NOT NULL)", []).map_err(|_| Error::ExecuteDatabaseCommandFailed("Failed to create the tasks table.".to_string()))?;
        Ok(DatabaseHandler { connection })
    }

    pub fn add_task(&self, id: u32, task: Task) -> Result<()> {
        self.connection
            .execute(
                "INSERT INTO tasks (id, status, title, priority) VALUES (?1, ?2, ?3, ?4)",
                (
                    id,
                    task.status.to_string(),
                    &task.title,
                    task.priority.to_string(),
                ),
            )
            .map_err(|_| {
                Error::ExecuteDatabaseCommandFailed(
                    "Failed to insert a new task instance into the tasks table.".to_string(),
                )
            })?;
        Ok(())
    }

    pub fn update_task(&self, id: u32, task: Task) -> Result<()> {
        self.connection
            .execute(
                "UPDATE tasks SET status = ?2, title = ?3, priority = ?4 WHERE id = ?1",
                (
                    id,
                    task.status.to_string(),
                    &task.title,
                    task.priority.to_string(),
                ),
            )
            .map_err(|_| {
                Error::ExecuteDatabaseCommandFailed(format!(
                    "Failed to update the task: {} with parameters: {:?}",
                    id, task
                ))
            })?;
        Ok(())
    }

    pub fn remove_task(&self, id: u32) -> Result<()> {
        self.connection.execute("DELETE FROM tasks WHERE id = ?1", [id]).map_err(|_| Error::ExecuteDatabaseCommandFailed(format!("Failed to delete the task: {}.", id)))?;
        Ok(())
    }
}
