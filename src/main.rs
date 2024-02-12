use fltk::{app, prelude::*, window::Window};
mod parser;
use parser::parser;

fn main() {
    // let app = app::App::default();
    // let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    // parser();
    // wind.end();
    // wind.show();
    // app.run().unwrap();
    parser();
}
