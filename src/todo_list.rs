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
                "{no} [{completed}] - {title}{description}",
                no = index,
                completed = match item.completed {
                    true => 'x',
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
