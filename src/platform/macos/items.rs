extern crate cocoa;

use self::cocoa::base::{class, id, nil};
use self::cocoa::foundation::NSString;
use self::cocoa::appkit::NSView;

use super::super::super::pane::Item;

use super::util;

pub struct ItemComponent {
    pub view: id, // NSView
    text_field: id, // NSTextField
}

impl ItemComponent {
    pub fn new() -> ItemComponent {
        let view = unsafe { NSView::alloc(nil).init() };

        let text_field = unsafe { msg_send![class("NSTextField"), new] };
        unsafe { view.addSubview_(text_field) };

        ItemComponent {
            view: view,
            text_field: text_field,
        }
    }

    pub fn render(&mut self, item: Item) {
        debug!("ItemComponent#render");
        util::resize_to_superview(self.view);
        util::resize_to_superview(self.text_field);

        let buffer_as_string = item.buffer.to_string();

        let attributed_string: id = unsafe {
            let string = NSString::alloc(nil).init_str(&buffer_as_string);
            let ptr: id = msg_send![class("NSMutableAttributedString"), alloc];
            msg_send![ptr, initWithString:string]
        };

        unsafe {
            msg_send![
                self.text_field,
                setAttributedStringValue:attributed_string
            ]
        };
    }
}
