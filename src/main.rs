mod command;
mod data_container;
mod todo_item;
mod todo_list;
mod traits;

use command::Command;
use data_container::DataContainer;
use std::env;
use std::path::Path;
use todo_list::TodoList;

fn do_command(
    command: Command,
    todo_list: &mut TodoList,
    data_container: &DataContainer,
) -> std::io::Result<()> {
    match command {
        Command::Get => (),
        Command::Add(task) => {
            todo_list.add_to_list(&task);
            data_container.save(todo_list)?;
        }
        Command::Describe(index, description) => {
            todo_list.add_description(index, description);
            data_container.save(todo_list)?;
        }
        Command::Done(index) => {
            todo_list.mark_done(index);
            data_container.save(todo_list)?;
        }
        Command::Remove(index) => {
            todo_list.remove_task(index);
            data_container.save(todo_list)?;
        }
    };
    todo_list.print();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Command::from(env::args());
    let data_path = Path::new("./data");
    let mut todo_list = match data_path.exists() {
        true => TodoList::from(data_path),
        false => TodoList::new(),
    };
    let data_container = DataContainer::new(data_path);
    do_command(command, &mut todo_list, &data_container)?;
    Ok(())
}
