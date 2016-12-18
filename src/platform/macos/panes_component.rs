extern crate cocoa;

use super::super::super::pane::Pane;

use self::cocoa::base::{id, nil};
use self::cocoa::appkit::NSView;

pub struct PanesComponent {
    view: id, // NSView
    current_panes: Vec<Box<Pane>>,
    current_pane_components: Vec<PaneComponent>,
}

impl PanesComponent {
    pub fn new() -> PanesComponent {
        PanesComponent {
            view: unsafe { NSView::alloc(nil).init() },
            current_panes: vec![],
            current_pane_components: vec![],
        }
    }

    pub fn render(&mut self, panes: Vec<Box<Pane>>) {
        if self.current_panes != panes {
            self.reset_panes(panes)
        }
    }

    fn reset_panes(&mut self, panes: Vec<Box<Pane>>) {
        // Reset all pane components

        self.current_panes = panes
    }
}

pub struct PaneComponent {
    view: id, // NSView
    pane: Box<Pane>,
}
