mod command;
mod todo_item;
mod todo_list;

use command::Command;
use std::env;
use todo_list::TodoList;

fn main() {
    let command = Command::from(env::args());
    let mut todo_list = TodoList::new();
    todo_list.add_to_list("Say hi to CJ");
    todo_list.add_to_list("Say hi to ch");

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(&task);
            todo_list.print();
        }
        Command::Describe(index, description) => {
            todo_list.add_description(index, description);
            todo_list.print();
        }
        Command::Done(index) => {
            todo_list.mark_done(index);
            todo_list.print();
        }
        Command::Remove(index) => {
            todo_list.remove_task(index);
            todo_list.print();
        }
    };
}
