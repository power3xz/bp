use chrono::{DateTime, Utc};

use crate::{
    repo::{MemoryRepo, Repo},
    task::Task,
};

#[derive(Debug)]
pub struct App {
    repo: Box<dyn Repo>,
}

impl App {
    pub fn new() -> Self {
        Self {
            repo: Box::new(MemoryRepo::new()),
        }
    }
    pub fn add(&mut self, title: &str) {
        self.repo.save_task_from_title(title);
    }

    pub fn get_list(&self) -> Vec<&Task> {
        self.repo.get_list().to_vec()
    }

    pub fn get(&self, id: u32) -> Option<&Task> {
        self.repo.get(id)
    }

    pub fn remove(&mut self, id: u32) {
        self.repo.remove(id)
    }

    pub fn update_title(&mut self, id: u32, title: &str) {
        self.repo.update_title(id, title)
    }

    pub fn update_description(&mut self, id: u32, description: &str) {
        self.repo.update_description(id, description)
    }

    pub fn update_due(&mut self, id: u32, due: DateTime<Utc>) {
        self.repo.update_due(id, due);
    }

    pub fn toggle_status(&mut self, id: u32) {
        self.repo.toggle_status(id);
    }

    pub fn update_priority(&mut self, id: u32, priority: u8) {
        self.repo.update_priority(id, priority)
    }
}
