use crate::{
    repo::{FileRepo, Repo},
    task::Task,
};

#[derive(Debug)]
pub struct App {
    repo: FileRepo,
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

    pub fn get_list(&self) -> &[Task] {
        self.repo.get_list()
    }
}
