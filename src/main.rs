#![allow(dead_code)]

extern crate env_logger;
extern crate freetype;
#[macro_use] extern crate log;
#[macro_use] extern crate objc;

pub mod buffer;
pub mod pane;
pub mod platform;
pub mod project;
pub mod workspace;

use std::env;
use std::path::Path;
use std::sync::Arc;

fn main() {
    env_logger::init().unwrap();

    // println!("Hello, world!");

    // let file = buffer::File::from_path(Path::new("./Cargo.toml"));
    // let lines = file.read_lines();
    // println!("{:?}", lines);

    let application = Arc::new(platform::Application::new());

    let project = project::Project {
        directory: env::current_dir().unwrap(),
    };

    let mut workspace = workspace::Workspace::new(application.clone(), project);
    workspace.render();

    let path = Path::new("./src/main.rs");
    workspace.open_path(path);
    workspace.render();

    application.run();
}
