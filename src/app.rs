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
        self.repo.save_task_from_title(title);
    }

    pub fn get_list(&self) -> Vec<&Task> {
        self.repo.get_list().to_vec()
    }
}
