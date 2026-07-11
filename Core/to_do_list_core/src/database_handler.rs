use crate::common::{Priority, Status, Task};
use crate::error::*;
use rusqlite::Connection;

pub struct DatabaseHandler {
    connection: Connection,
}

impl DatabaseHandler {
    pub fn new() -> Result<Self> {
        let connection =
            Connection::open("task_database.db").map_err(|_| Error::OpenDatabaseFailed)?;
        connection.execute("CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY, status TEXT NOT NULL CHECK (status IN ('Planned', 'Done')), title TEXT, priority TEXT NOT NULL CHECK (priority IN ('Low', 'Medium', 'High')))", []).map_err(|_| Error::ExecuteDatabaseCommandFailed("Failed to create the tasks table.".to_string()))?;
        Ok(DatabaseHandler { connection })
    }

    // Commands
    pub fn add_task(&self, id: i64, task: Task) -> Result<()> {
        self.connection
            .execute(
                "INSERT INTO tasks (id, status, title, priority) VALUES (?1, ?2, ?3, ?4)",
                (id, &task.status, &task.title, &task.priority),
            )
            .map_err(|error| {
                Error::ExecuteDatabaseCommandFailed(format!(
                    "Failed to insert a new task instance into the tasks table: {:?}",
                    error
                ))
            })?;
        Ok(())
    }
    pub fn update_task(&self, id: i64, task: Task) -> Result<()> {
        self.connection
            .execute(
                "UPDATE tasks SET status = ?2, title = ?3, priority = ?4 WHERE id = ?1",
                (id, &task.status, &task.title, &task.priority),
            )
            .map_err(|error| {
                Error::ExecuteDatabaseCommandFailed(format!(
                    "Failed to update the task: {} with parameters: {:?}, due to error: {:?}",
                    id, task, error
                ))
            })?;
        Ok(())
    }
    pub fn remove_task(&self, id: i64) -> Result<()> {
        self.connection
            .execute("DELETE FROM tasks WHERE id = ?1", [id])
            .map_err(|error| {
                Error::ExecuteDatabaseCommandFailed(format!(
                    "Failed to delete the task: {}, due to error: {:?}",
                    id, error
                ))
            })?;
        Ok(())
    }

    // Queries
    pub fn get_tasks_list(&self) -> Result<String> {
        let mut response = String::new();
        let mut statement = self
            .connection
            .prepare("SELECT id, status, title, priority FROM tasks")
            .map_err(|error| {
                Error::PrepareDatabaseQueryFailed(format!(
                    "Failed to get the tasks list, due to error: {:?}",
                    error
                ))
            })?;
        let rows = statement.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, Status>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Priority>(3)?,
            ))
        })?;
        for row in rows {
            let (id, status, title, priority) = row?;
            response.push_str(
                format!(
                    "id: {}, status: {}, title: {}, priority: {}\n",
                    id, status, title, priority
                )
                .as_str(),
            );
        }
        Ok(response)
    }
}
