use std::fs;
use std::io::{BufRead, BufReader};
use std::path;

pub struct File {
    path: path::PathBuf,
    file: fs::File,
}

impl File {
    pub fn from_path(path: &path::Path) -> File {
        let file = fs::File::open(path).expect("could not open file");

        File {
            path: path.to_path_buf(),
            file: file,
        }
    }

    /// Read the whole file into lines.
    fn read_lines(&self) -> Vec<Line> {
        let ref file = self.file;
        let reader = BufReader::new(file);

        reader.lines()
            .filter(|line| {
                match line {
                    &Ok(_) => true,
                    &Err(ref err) => {
                        println!("error reading line: {:?}", err);
                        false
                    }
                }
            })
            .map(|line| {
                let chars = line.unwrap().chars().collect();
                Line { chars: chars }
            })
            .collect()
    }
}


pub struct Line {
    chars: Vec<char>,
}

/// Internal storage of a file for editing.
pub struct Buffer {
    pub file: File,
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn from_file(file: File) -> Buffer {
        Buffer {
            lines: file.read_lines(),
            file: file,
        }
    }
}
