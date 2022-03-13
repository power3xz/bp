use std::fmt::{Debug, Display};

use crate::task::Task;

pub trait Repo {
    fn generate_id(&self) -> u32;
    fn save_task(&mut self, task: Task);
    fn save_task_from_title(&mut self, title: &str);
    fn get_list(&self) -> Vec<&Task>;
    fn get(&self, id: u32) -> Option<&Task>;
    fn update_title(&mut self, id: u32, title: &str);
}

#[derive(Debug)]
pub struct MemoryRepo {
    list: Vec<Task>,
}

impl MemoryRepo {
    pub fn new() -> Self {
        Self { list: vec![] }
    }
}
impl Display for dyn Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self.get_list()).as_str())
    }
}
impl Debug for dyn Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self::Display::fmt(self, f)
    }
}

impl Repo for MemoryRepo {
    fn generate_id(&self) -> u32 {
        match self.list.iter().map(|t| t.id).max() {
            Some(id) => id + 1,
            _ => 0,
        }
    }
    fn save_task(&mut self, task: Task) {
        self.list.push(task);
    }
    fn save_task_from_title(&mut self, title: &str) {
        self.save_task(Task::new(self.generate_id(), title));
    }
    fn get_list(&self) -> Vec<&Task> {
        self.list.iter().collect()
    }
    fn get(&self, id: u32) -> Option<&Task> {
        self.list.iter().find(|&t| t.id == id)
    }

    fn update_title(&mut self, id: u32, title: &str) {
        if let Some(index) = self.list.iter().position(|t| t.id == id) {
            self.list[index].title = title.to_string();
        }
    }
}
