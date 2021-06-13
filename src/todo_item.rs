pub struct TodoItem {
    pub name: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(name: String) -> Self {
        Self {
            name,
            completed: false,
        }
    }
}

impl From<&str> for TodoItem {
    fn from(name: &str) -> Self {
        Self::new(name.to_string())
    }
}
