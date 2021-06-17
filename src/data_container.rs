use std::fs;
use std::io::Write;
use std::path::Path;

use crate::todo_list::TodoList;
use crate::traits::ToCSV;

pub struct DataContainer<'a> {
    data_path: &'a Path,
}

impl<'a> DataContainer<'a> {
    pub fn new(data_path: &'a Path) -> Self {
        Self { data_path }
    }
    pub fn save(&self, todo_list: &TodoList) -> std::io::Result<()> {
        let mut f = match self.data_path.exists() {
            true => fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(self.data_path)?,
            _ => panic!(""),
        };
        f.write_all(todo_list.to_csv().as_bytes())?;
        f.flush()?;
        Ok(())
    }
}
