use buffer::Buffer;

pub struct Pane {
    items: Vec<Item>,
}

pub struct Item {
    buffer: Buffer,
}
