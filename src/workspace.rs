use std::cell::RefCell;
use std::path::Path;
use std::sync::{Arc, Weak};

use pane::{Item, Pane};
use project::Project;
use platform;

pub struct Workspace {
    /// Parent application reference.
    pub application: Weak<platform::Application>,
    pub project: Project,
    /// Window to which the `Workspace` is rendered.
    pub window: RefCell<platform::Window>,
    /// A workspace will have one or more panes in a given arrangement;
    /// multiple panes with arrangements isn't currently implemented, but
    /// for the sake of forward-thinking'ness we'll represent the
    /// workspace's panes as an array.
    pub panes: Vec<Box<Pane>>,
}

impl Workspace {
    pub fn new(application: Arc<platform::Application>, project: Project) -> Workspace {
        let window = platform::Window::new();

        Workspace {
            application: Arc::downgrade(&application),
            project: project,
            window: RefCell::new(window),
            panes: vec![],
        }
    }

    pub fn render(&self) {
        let panes = self.panes.clone();
        let project = self.project.clone();

        let mut window = self.window.borrow_mut();
        window.render(project, panes)
    }

    /// Gets the current active pane. If one does not exist then it will
    /// create one.
    pub fn active_pane_mut<'a>(&'a mut self) -> &'a mut Pane {
        if self.panes.is_empty() {
            self.panes.push(Box::new(Pane::new()))
        }

        &mut self.panes[0]
    }

    pub fn open_path<P: AsRef<Path>>(&mut self, path: P) {
        let mut pane = self.active_pane_mut();
        pane.add_item(Item::from_path(path))
    }
}
