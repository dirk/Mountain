extern crate cocoa;

use self::cocoa::base::{id, nil, selector};
use self::cocoa::foundation::{NSAutoreleasePool, NSString};
use self::cocoa::appkit::{self, NSApp, NSApplication, NSMenu, NSMenuItem};

use super::common;

pub struct Application {
    pool: id, // NSAutoreleasePool
    app: id, // NSApp
}

impl Application {
    pub fn new() -> Application {
        let pool = unsafe { NSAutoreleasePool::new(nil) };

        let app: id;
        unsafe {
            app = NSApp();
            app.setActivationPolicy_(appkit::NSApplicationActivationPolicyRegular);
        }

        let application = Application {
            pool: pool,
            app: app,
        };

        unsafe { application.setup_menu(); }

        application
    }

    // Boilerplate menu setup. Cribbed from:
    //   https://github.com/servo/cocoa-rs/blob/master/examples/hello_world.rs
    unsafe fn setup_menu(&self) {
        let main_menu = NSMenu::new(nil).autorelease();
        self.app.setMainMenu_(main_menu);

        let app_menu_item = NSMenuItem::new(nil).autorelease();
        main_menu.addItem_(app_menu_item);

        let app_menu = NSMenu::new(nil).autorelease();
        app_menu_item.setSubmenu_(app_menu);

        let quit_title = NSString::alloc(nil).init_str("Quit");
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            quit_title,
            quit_action,
            quit_key
        ).autorelease();
        app_menu.addItem_(quit_item);
    }

    // Start the event loop
    pub fn run(&self) {
        unsafe { self.app.run() }
    }
}

pub struct Menu {
    title: String,
    items: Vec<MenuItem>,
}

impl common::Menu for Menu {
    type MenuItem = MenuItem;

    fn new(title: String, items: Option<Vec<MenuItem>>) -> Menu {
        Menu {
            title: title,
            items: items.unwrap_or(vec![]),
        }
    }
}

pub struct MenuItem {
    title: String,
}
