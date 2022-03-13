use crate::{
    repo::{FileRepo, Repo},
    task::Task,
};

#[derive(Debug)]
pub struct App {
    pub repo: FileRepo,
}

impl App {
    pub fn new() -> Self {
        Self {
            repo: FileRepo::new(),
        }
    }
    pub fn add(&mut self, title: &str) {
        self.repo.save_task(Task::new(title));
    }
}
