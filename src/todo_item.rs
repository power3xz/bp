pub struct TodoItem {
    pub name: String,
    pub completed: bool,
    pub description: Option<String>,
    pub priority: Option<u8>,
}

impl TodoItem {
    pub fn new(name: String) -> Self {
        Self {
            name,
            completed: false,
            description: None,
            priority: None,
        }
    }
}

impl From<&str> for TodoItem {
    fn from(name: &str) -> Self {
        Self::new(name.to_string())
    }
}
