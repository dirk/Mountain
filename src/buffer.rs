use std::fs;
use std::io::{BufRead, BufReader};
use std::path;

#[derive(Debug)]
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
    pub fn read_lines(&self) -> Vec<Line> {
        let ref file = self.file;
        let mut reader = BufReader::new(file);
        let mut lines: Vec<Line> = vec![];

        loop {
            let mut buf = String::new();

            match reader.read_line(&mut buf) {
                Ok(0) => break, // No more input
                Ok(_) => {
                    let chars = buf.chars().collect();
                    lines.push(Line { chars: chars })
                },
                Err(e) => panic!(e),
            }
        }

        lines
    }
}

#[derive(Debug)]
pub struct Line {
    chars: Vec<char>,
}

#[derive(Debug)]
pub enum BufferSource {
    File(File),
    Unknown,
}

/// Internal storage of a file for editing.
#[derive(Debug)]
pub struct Buffer {
    pub source: BufferSource,
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn from_file(file: File) -> Buffer {
        Buffer {
            lines: file.read_lines(),
            source: BufferSource::File(file),
        }
    }
}
