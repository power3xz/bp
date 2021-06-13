use std::env::Args;

pub enum Command {
    Get,
    Add(String),
    Done(usize),
    Remove(usize),
}

impl From<Args> for Command {
    fn from(args: Args) -> Self {
        let arguments: Vec<String> = args.collect();
        match arguments[1].as_str() {
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
        }
    }
}
