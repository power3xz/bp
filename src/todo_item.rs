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

    pub fn to_csv(&self) -> String {
        format!(
            "{completed},{name},{description},{priority}",
            completed = match self.completed {
                true => "o",
                false => "x",
            },
            name = self.name,
            description = match &self.description {
                Some(desc) => desc,
                None => "",
            },
            priority = match self.priority {
                Some(p) => p.to_string(),
                None => "".to_string(),
            }
        )
    }
}

impl From<&str> for TodoItem {
    fn from(name: &str) -> Self {
        Self::new(name.to_string())
    }
}
