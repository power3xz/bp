use std::fs;
use std::path::Path;

use crate::todo_item::TodoItem;

pub struct TodoList {
    pub list: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn add_to_list(&mut self, task: &str) {
        self.list.push(task.into());
    }

    pub fn add_item_to_list(&mut self, task: TodoItem) {
        self.list.push(task);
    }

    pub fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!(
                "{no} [{completed}] - {title}{description}",
                no = index,
                completed = match item.completed {
                    true => 'v',
                    false => ' ',
                },
                title = item.name,
                description = match &item.description {
                    Some(desc) => format!(" {}", desc),
                    None => "".to_string(),
                }
            )
        }
    }

    pub fn mark_done(&mut self, index: usize) {
        self.list[index].completed = !self.list[index].completed;
    }

    pub fn remove_task(&mut self, index: usize) {
        self.list.remove(index);
    }

    pub fn add_description(&mut self, index: usize, description: String) {
        self.list[index].description = Some(description);
    }
}

impl From<&Path> for TodoList {
    fn from(path: &Path) -> Self {
        let content = fs::read_to_string(path).expect("file not found");
        let mut list = TodoList::new();
        for line in content.lines() {
            let fields: Vec<&str> = line.split(',').collect();
            let item = TodoItem {
                completed: match *fields.get(0).expect("completed is not exists") {
                    "o" => true,
                    "x" => false,
                    _ => false,
                },
                name: fields.get(1).expect("name is not exists").to_string(),
                description: Some(fields.get(2).unwrap_or(&"").to_string()),
                priority: None,
            };

            list.add_item_to_list(item);
        }
        list
    }
}
