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
    pub window: platform::Window,
    /// A workspace will have one or more panes in a given arrangement;
    /// multiple panes with arrangements isn't currently implemented, but
    /// for the sake of forward-thinking'ness we'll represent the
    /// workspace's panes as an array.
    pub panes: Vec<Pane>,
}

impl Workspace {
    pub fn new(application: Arc<platform::Application>, project: Project) -> Workspace {
        let window = platform::Window::new();

        Workspace {
            application: Arc::downgrade(&application),
            project: project,
            window: window,
            panes: vec![],
        }
    }

    pub fn render(&self) {
        let panes = self.panes.clone();
        let project = self.project.clone();
        self.window.render(project, panes)
    }

    pub fn active_pane_mut<'a>(&'a mut self) -> &'a mut Pane {
        &mut self.panes[0]
    }

    pub fn open_path<P: AsRef<Path>>(&mut self, path: P) {
        let mut pane = self.active_pane_mut();
        pane.add_item(Item::from_path(path))
    }
}
