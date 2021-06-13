mod command;
mod todo_item;
mod todo_list;

use command::Command;
use std::env;
use todo_list::TodoList;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();
    todo_list.add_to_list("Say hi to CJ");
    todo_list.add_to_list("Say hi to ch");
    let command = match arguments[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(arguments[2].clone()),
        "done" => Command::Done(
            arguments[2]
                .parse::<usize>()
                .expect("error converting to int"),
        ),
        "remove" => Command::Remove(
            arguments[2]
                .parse::<usize>()
                .expect("error converting to int"),
        ),
        _ => panic!("you must provide an accepted command"),
    };

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_to_list(&task);
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
