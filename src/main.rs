#![allow(dead_code)]

extern crate freetype;

pub mod buffer;
pub mod pane;
pub mod platform;
pub mod project;
pub mod workspace;

use std::env;
use std::sync::Arc;

fn main() {
    // println!("Hello, world!");

    // use std::path::Path;
    //
    // let file = buffer::File::from_path(Path::new("./Cargo.toml"));
    // let lines = file.read_lines();
    // println!("{:?}", lines);

    let application = Arc::new(platform::Application::new());

    let project = project::Project {
        directory: env::current_dir().unwrap(),
    };

    let workspace = workspace::Workspace::new(application.clone(), project);
    workspace.render();

    application.run();
}
