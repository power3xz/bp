use app::App;

mod app;
mod repo;
mod task;

fn main() {
    let mut app = App::new();
    app.add("하이염!1");
    app.add("하이염!2");
    app.add("하이염!3");
    app.add("하이염!4");
    app.add("하이염!5");
    for task in app.get_list() {
        println!("{:?}", task);
    }
    app.update_title(1, "하이지요");
    println!("{:?}", app.get(1));
    app.remove(1);
    println!("{:?}", app.get(1));
    app.update_priority(2, 2);
    app.toggle_status(2);
    println!("{:?}", app.get(2));
}
