use std::env::Args;

pub enum Command {
    Get,
    Add(String),
    Done(usize),
    Remove(usize),
    Describe(usize, String),
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
                    .expect("error converting to integer"),
            ),
            "remove" => Command::Remove(
                arguments[2]
                    .parse::<usize>()
                    .expect("error converting to integer"),
            ),
            "describe" => Command::Describe(
                arguments[2]
                    .parse::<usize>()
                    .expect("error converting to integer"),
                arguments[3].clone(),
            ),
            _ => panic!("you must provide an accepted command"),
        }
    }
}
