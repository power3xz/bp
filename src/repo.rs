use crate::task::Task;

pub trait Repo {
    fn save_task(&mut self, task: Task);
    fn get_list(&self) -> &[Task];
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
    fn save_task(&mut self, task: Task) {
        self.list.push(task);
    }
    fn get_list(&self) -> &[Task] {
        &self.list[..]
    }
}
