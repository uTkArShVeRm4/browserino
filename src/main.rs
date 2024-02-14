use fltk::{app, prelude::*, window::Window};
mod parser;
use parser::parser;
use std::fs;

fn main() {
    // let app = app::App::default();
    // let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    // parser();
    // wind.end();
    // wind.show();
    // app.run().unwrap();
    parser();
    let contents = fs::read_to_string("browser/index.html").unwrap();
    print!("{}", contents);
}
