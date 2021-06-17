use crate::todo_list::TodoList;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn save(todo_list: &TodoList, data_path: &Path) -> std::io::Result<()> {
    let mut f = match data_path.exists() {
        true => fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(data_path)?,
        false => fs::File::create(data_path)?,
    };
    for item in &todo_list.list {
        f.write_all(format!("{}\n", item.to_csv()).as_bytes())?;
    }
    f.flush()?;
    Ok(())
}
