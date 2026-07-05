use std::collections::HashMap;
use crate::common::{Priority, Status, Task};

pub struct AppState {
    port: String,
    tasks: HashMap<u32, Task>,
}

impl AppState {
    pub fn new(port: &str) -> Self {
        Self {
            port: port.to_string(),
            tasks: HashMap::new(),
        }
    }

    pub fn get_port(&self) -> String {
        format!("Currently running on port: {}\n", self.port)
    }

    pub fn get_tasks(&self) -> String {
        self.tasks.iter().map(|(id, t)| format!("{}, {}, {}, {}", id, t.title, t.priority, t.status)).collect::<Vec<String>>().join("\n")
    }

    pub fn add_task(&mut self, title: String, priority: Priority) {
        let id = self.get_free_id(0);
        self.tasks.insert(id, Task {
            status: Status::Planned,
            title,
            priority,
        });
    }

    pub fn update_task_status(&mut self, id: u32, status: Status) {
        if let Some (old_task) = self.tasks.get_mut(&id) {
            old_task.status = status
        }
    }

    pub fn update_task_title(&mut self, id: u32, title: String) {
        if let Some (old_task) = self.tasks.get_mut(&id) {
            old_task.title = title
        }
    }

    pub fn update_task_priority(&mut self, id: u32, priority: Priority) {
        if let Some (old_task) = self.tasks.get_mut(&id) {
            old_task.priority = priority
        }
    }
    
    pub fn delete_task(&mut self, id: u32) {
        self.tasks.remove(&id);
    }

    fn get_free_id(&self, proposed_id: u32) -> u32 {
        if self.tasks.iter().all(|(id, _)| id != &proposed_id) {
            proposed_id
        } else {
            self.get_free_id(proposed_id + 1)
        }
    }
}