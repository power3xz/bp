use crate::{repo::Repo, task::Task};

pub struct BP {
    pub repo: Repo,
}

impl BP {
    pub fn new() -> Self {
        Self { repo: Repo }
    }
    pub fn add(&mut self, title: &str) {
        self.repo.save_task(Task::new(title));
    }
}
