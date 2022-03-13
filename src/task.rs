use chrono::prelude::*;

#[derive(Debug)]
pub enum Status {
    Normal,
    Done,
}

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: Status,
    pub description: Option<String>,
    pub due: Option<DateTime<Utc>>,
    pub priority: u8,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Self {
            id: 0,
            title: title.to_string(),
            status: Status::Normal,
            description: None,
            due: None,
            priority: 0,
        }
    }

    pub fn toggle(&mut self) {
        match self.status {
            Status::Normal => self.status = Status::Done,
            Status::Done => self.status = Status::Normal,
        }
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn set_due(&mut self, due: DateTime<Utc>) {
        self.due = Some(due);
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }
}
