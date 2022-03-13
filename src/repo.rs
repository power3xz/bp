use crate::task::Task;

pub trait Repo {
    fn generate_id(&self) -> u32;
    fn save_task(&mut self, title: &str);
    fn get_list(&self) -> Vec<&Task>;
}

#[derive(Debug)]
pub struct FileRepo {
    list: Vec<Task>,
}

impl FileRepo {
    pub fn new() -> Self {
        Self { list: vec![] }
    }
}

impl Repo for FileRepo {
    fn generate_id(&self) -> u32 {
        match self.list.iter().map(|t| t.id).max() {
            Some(id) => id + 1,
            _ => 0,
        }
    }
    fn save_task(&mut self, task: &str) {
        self.list.push(Task::new(self.generate_id(), task));
    }
    fn get_list(&self) -> Vec<&Task> {
        self.list.iter().collect()
    }
}
