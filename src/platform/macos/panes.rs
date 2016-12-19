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

        for pane in self.current_pane_components.iter() {
            pane.render()
        }
    }

    // Clear the current pane components and re-add them.
    // TODO: Make this reconcile the current panes with the new panes and
    //   intelligently determine the added/removed ones.
    fn reset_panes(&mut self, panes: Vec<Box<Pane>>) {
        for component in &self.current_pane_components {
            unsafe { component.view.removeFromSuperview() }
        }

        self.current_panes = panes.clone();

        self.current_pane_components = panes.iter().map(|pane| {
            PaneComponent::new(pane.clone())
        }).collect();

        for component in &self.current_pane_components {
            unsafe { self.view.addSubview_(component.view) }
        }
    }
}

pub struct PaneComponent {
    view: id, // NSView
    pane: Box<Pane>,
}

impl PaneComponent {
    pub fn new(pane: Box<Pane>) -> PaneComponent {
        PaneComponent {
            view: unsafe { NSView::alloc(nil).init() },
            pane: pane,
        }
    }

    pub fn render(&self) {}
}
