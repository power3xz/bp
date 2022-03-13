use app::App;

mod app;
mod repo;
mod task;

fn main() {
    let mut app = App::new();
    app.add("하이염!");
    app.add("하이염!");
    app.add("하이염!");
    app.add("하이염!");
    app.add("하이염!");
    for task in app.get_list() {
        println!("{:?}", task);
    }
}
