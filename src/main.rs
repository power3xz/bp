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
        for (index, item) in self.list.iter().enumerate() {
            println!("{} [{}] - {}", index, item.completed, item.name)
        }
    }

    fn mark_done(&mut self, index: usize) {
        self.list[index].completed = 'x';
    }
}

enum Command {
    Get,
    Add(String),
    Done(usize),
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();
    todo_list.add_to_list("Say hi to CJ");
    todo_list.add_to_list("Say hi to ch");
    let command = match arguments[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(arguments[2].clone()),
        "done" => Command::Done(
            arguments[2]
                .parse::<usize>()
                .expect("error converting to int"),
        ),
        _ => panic!("you must provide an accepted command"),
    };

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(&task);
            todo_list.print();
        }
        Command::Done(index) => {
            todo_list.mark_done(index);
            todo_list.print();
        }
    };
}
