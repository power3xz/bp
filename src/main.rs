use std::env;
struct TodoItem {
    name: String,
    completed: char,
}

impl TodoItem {
    fn new(name: String) -> Self {
        Self {
            name,
            completed: ' ',
        }
    }
}

impl From<&str> for TodoItem {
    fn from(name: &str) -> Self {
        Self::new(name.to_string())
    }
}

struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> Self {
        Self { list: vec![] }
    }

    fn add_to_list(&mut self, name: &str) {
        self.list.push(name.into());
    }

    fn print(&self) {
        for item in &self.list {
            println!("[{}] - {}", item.completed, item.name)
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();
    let mut todo_list = TodoList::new();
    todo_list.add_to_list("Say hi to CJ");
    todo_list.add_to_list("Say hi to ch");

    if command == "get" {
        todo_list.print();
    } else if command == "add" {
        let task = arguments[2].clone();
        todo_list.add_to_list(&task);
        todo_list.print();
    }
    println!("{:#?}", arguments);
}
