#![allow(improper_ctypes)]

extern crate cocoa;
extern crate core_graphics;

use self::cocoa::base::{class, id, nil, YES};
use self::cocoa::appkit::NSView;
use self::core_graphics::base::CGFloat;

use super::super::super::pane::Pane;

use super::util;

pub struct PanesComponent {
    pub view: id, // NSView
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
        debug!("PanesComponent#render");
        if self.current_panes != panes {
            self.reset_panes(panes)
        }

        util::resize_to_superview(self.view);

        for (i, pane_component) in self.current_pane_components.iter().enumerate() {
            let ref pane = self.current_panes[i];
            pane_component.render(pane.clone());
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
    pub view: id, // NSView
    pane: Box<Pane>,
}

impl PaneComponent {
    pub fn new(pane: Box<Pane>) -> PaneComponent {
        PaneComponent {
            view: unsafe { NSView::alloc(nil).init() },
            pane: pane,
        }
    }

    pub fn render(&self, pane: Box<Pane>) {
        debug!("PaneComponent#render");
        debug_assert_eq!(self.pane, pane);
        util::resize_to_superview(self.view);

        self.render_background();
    }

    fn render_background(&self) {
        let layer = unsafe { msg_send![class("CALayer"), layer] };
        let gray_color = unsafe {
            CGColorCreateGenericGray(0.5, 1.)
        };

        unsafe {
            self.view.setLayer(layer);
            self.view.setWantsLayer(YES);
        };

        unsafe {
            msg_send![
                layer,
                setBackgroundColor:gray_color
            ]
        };
    }
}

#[repr(C)]
pub struct __CGColor;

pub type CGColorRef = *const __CGColor;

#[link(name = "CoreGraphics", kind = "framework")]
extern {
    fn CGColorCreateGenericGray(gray: CGFloat, alpha: CGFloat) -> CGColorRef;
}
