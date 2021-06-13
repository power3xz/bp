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

    fn add_to_list(&mut self, task: &str) {
        self.list.push(task.into());
    }

    fn print(&self) {
        for item in &self.list {
            println!("[{}] - {}", item.completed, item.name)
        }
    }
}

enum Command {
    Get,
    Add(String),
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();
    todo_list.add_to_list("Say hi to CJ");
    todo_list.add_to_list("Say hi to ch");
    let command = match arguments[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(arguments[2].clone()),
        _ => panic!("you must provide an accepted command"),
    };

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(&task);
            todo_list.print();
        }
    };
    println!("{:#?}", arguments);
}
