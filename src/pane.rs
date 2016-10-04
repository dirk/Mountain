use std::convert::AsRef;
use std::path::Path;

use buffer::{Buffer, File};

pub struct Pane {
    items: Vec<Item>,
}

impl Pane {
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item)
    }
}

pub struct Item {
    buffer: Buffer,
}

impl Item {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Item {
        let buffer = Buffer::from_file(File::from_path(path.as_ref()));

        Item {
            buffer: buffer,
        }
    }
}
