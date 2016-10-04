use std::path::{Path, PathBuf};

use pane::{Item, Pane};

struct Project {
    directory: PathBuf,
}

struct Window;

struct Workspace {
    project: Project,
    /// Window to which the `Workspace` is rendered.
    window: Window,
    panes: [Pane; 1],
}

impl Workspace {
    pub fn active_pane_mut<'a>(&'a mut self) -> &'a mut Pane {
        &mut self.panes[0]
    }

    pub fn open_path<P: AsRef<Path>>(&mut self, path: P) {
        let mut pane = self.active_pane_mut();
        pane.add_item(Item::from_path(path))
    }
}
