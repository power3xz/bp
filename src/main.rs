use chrono::Utc;

use crate::task::Task;

mod task;

fn main() {
    let mut task = Task::new("테스트 입니다!");
    println!("{:?}", task);
    task.toggle();
    println!("{:?}", task);
    task.set_due(Utc::now());
    println!("{:?}", task);
}
