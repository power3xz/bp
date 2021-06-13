mod command;
mod todo_item;
mod todo_list;

use command::Command;
use std::env;
use std::io::Write;
use std::path::Path;
use todo_list::TodoList;

fn save(todo_list: &TodoList, data_path: &Path) -> std::io::Result<()> {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(data_path)?;
    for item in &todo_list.list {
        f.write_all(format!("{}\n", item.to_csv()).as_bytes())?;
    }
    f.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Command::from(env::args());
    let data_path = Path::new("data");
    let mut todo_list = match data_path.exists() {
        true => TodoList::from(data_path),
        false => TodoList::new(),
    };

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(&task);
            todo_list.print();
            save(&todo_list, data_path)?;
        }
        Command::Describe(index, description) => {
            todo_list.add_description(index, description);
            todo_list.print();
            save(&todo_list, data_path)?;
        }
        Command::Done(index) => {
            todo_list.mark_done(index);
            todo_list.print();
            save(&todo_list, data_path)?;
        }
        Command::Remove(index) => {
            todo_list.remove_task(index);
            todo_list.print();
            save(&todo_list, data_path)?;
        }
    };
    Ok(())
}
