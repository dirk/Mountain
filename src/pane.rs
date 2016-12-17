use std::convert::AsRef;
use std::path::Path;
use std::sync::Arc;

use buffer::{Buffer, File};

#[derive(Clone)]
pub struct Pane {
    items: Vec<Item>,
}

impl Pane {
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item)
    }
}

#[derive(Clone)]
pub struct Item {
    // As an optimization `Buffer`'s are managed separately and are
    // internally mutable.
    buffer: Arc<Buffer>,
}

impl Item {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Item {
        let buffer = Buffer::from_file(File::from_path(path.as_ref()));

        Item {
            buffer: Arc::new(buffer),
        }
    }
}
