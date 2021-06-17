mod command;
mod data_container;
mod todo_item;
mod todo_list;

use command::Command;
use std::env;
use std::path::Path;
use todo_list::TodoList;

fn do_command(command: Command, data_path: &Path) -> std::io::Result<()> {
    let mut todo_list = match data_path.exists() {
        true => TodoList::from(data_path),
        false => TodoList::new(),
    };
    match command {
        Command::Get => (),
        Command::Add(task) => {
            todo_list.add_to_list(&task);
            data_container::save(&todo_list, data_path)?;
        }
        Command::Describe(index, description) => {
            todo_list.add_description(index, description);
            data_container::save(&todo_list, data_path)?;
        }
        Command::Done(index) => {
            todo_list.mark_done(index);
            data_container::save(&todo_list, data_path)?;
        }
        Command::Remove(index) => {
            todo_list.remove_task(index);
            data_container::save(&todo_list, data_path)?;
        }
    };
    todo_list.print();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Command::from(env::args());
    let data_path = Path::new("./data");
    do_command(command, data_path)?;
    Ok(())
}
