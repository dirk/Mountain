use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};

use platform;
use pane::{Item, Pane};

pub struct Project {
    pub directory: PathBuf,
}

pub struct Window {
    /// Native window instance.
    window: platform::Window,
}

impl Window {
    pub fn new() -> Window {
        Window {
            window: platform::Window::new(),
        }
    }

    pub fn set_title<T: AsRef<str>>(&self, title: T) {
        self.window.set_title(title)
    }
}

pub struct Workspace {
    /// Parent application reference.
    pub application: Weak<platform::Application>,
    pub project: Project,
    /// Window to which the `Workspace` is rendered.
    pub window: Window,
    /// A workspace will have one or more panes in a given arrangement;
    /// multiple panes with arrangements isn't currently implemented, but
    /// for the sake of forward-thinking'ness we'll represent the
    /// workspace's panes as an array.
    pub panes: Vec<Pane>,
}

impl Workspace {
    pub fn new(application: Arc<platform::Application>, project: Project) -> Workspace {
        let window = Window::new();

        if let Some(name) = project.directory.file_name().and_then(|s| s.to_str()) {
            window.set_title(name)
        }

        Workspace {
            application: Arc::downgrade(&application),
            project: project,
            window: window,
            panes: vec![],
        }
    }

    pub fn active_pane_mut<'a>(&'a mut self) -> &'a mut Pane {
        &mut self.panes[0]
    }

    pub fn open_path<P: AsRef<Path>>(&mut self, path: P) {
        let mut pane = self.active_pane_mut();
        pane.add_item(Item::from_path(path))
    }
}
