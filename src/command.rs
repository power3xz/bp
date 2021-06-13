pub enum Command {
    Get,
    Add(String),
    Done(usize),
    Remove(usize),
}
