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

    pub fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!(
                "{} [{}] - {}",
                index,
                match item.completed {
                    true => 'x',
                    false => ' ',
                },
                item.name
            )
        }
    }

    pub fn mark_done(&mut self, index: usize) {
        self.list[index].completed = !self.list[index].completed;
    }

    pub fn remove_task(&mut self, index: usize) {
        self.list.remove(index);
    }
}
