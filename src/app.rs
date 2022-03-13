use crate::{
    repo::{FileRepo, Repo},
    task::Task,
};

#[derive(Debug)]
pub struct App {
    repo: Box<dyn Repo>,
}

impl App {
    pub fn new() -> Self {
        Self {
            repo: Box::new(FileRepo::new()),
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
}
