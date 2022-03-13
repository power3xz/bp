use bp::App;

mod bp;
mod repo;
mod task;

fn main() {
    let mut app = App::new();
    app.add("하이염!");
    println!("{:?}", app);
}
