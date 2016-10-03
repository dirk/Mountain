use buffer::Buffer;

pub struct Pane {
    itesm: Vec<Item>,
}

pub struct Item {
    buffer: Buffer,
}
