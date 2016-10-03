use std::fs;
use std::path;
use std::sync::{Arc, Mutex};

pub struct File {
    path: path::PathBuf,
    file: fs::File,
}

pub struct Line {
    chars: Vec<char>,
}

/// Internal storage of a file for editing.
pub struct Buffer {
    pub lines: Vec<Line>,
    pub file: File,
}
