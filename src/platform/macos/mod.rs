extern crate cocoa;

use self::cocoa::base::{id, nil, selector, NO};
use self::cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString, NSUInteger};
use self::cocoa::appkit::{self, NSApp, NSApplication, NSMenu, NSMenuItem, NSRunningApplication, NSWindow};

use super::common;
use super::super::pane::Pane;
use super::super::project::Project;

mod items;
mod panes;
mod util;

use self::panes::PanesComponent;

pub struct Application {
    pool: id, // NSAutoreleasePool
    app: id, // NSApplication
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

    /// Bring the application to the front (as we've just been launched) and
    /// start the event loop
    pub fn run(&self) {
        use self::cocoa::appkit::NSApplicationActivateIgnoringOtherApps;

        unsafe {
            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

            self.app.run();
        }
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

pub struct Window {
    window: id, // NSWindow
    panes: PanesComponent,
}

impl Window {
    pub fn new() -> Window {
        use self::cocoa::appkit::*;

        // Window setup stuff cribbed from:
        //   https://github.com/servo/cocoa-rs/blob/5f5eece/examples/hello_world.rs#L42-L55

        let style_mask =
            (NSTitledWindowMask as NSUInteger) |
            (NSClosableWindowMask as NSUInteger) |
            (NSMiniaturizableWindowMask as NSUInteger) |
            (NSResizableWindowMask as NSUInteger);

        let window = unsafe {
            NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
                style_mask,
                NSBackingStoreBuffered,
                NO
            ).autorelease()
        };

        let panes = PanesComponent::new();

        unsafe {
            window.setContentView_(panes.view);
            window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
            window.makeKeyAndOrderFront_(nil);
        };

        Window {
            window: window,
            panes: panes,
        }
    }

    pub fn render(&mut self, project: Project, panes: Vec<Box<Pane>>) {
        if let Some(name) = project.directory.file_name().and_then(|s| s.to_str()) {
            self.set_title(name)
        }

        let content_view = self.panes.view;
        let mut bounds: NSRect = unsafe { self.window.frame() };
        bounds.origin = NSPoint::new(0., 0.);
        let contect_rect: NSRect = unsafe { self.window.contentRectForFrameRect_(bounds) };
        unsafe {
            msg_send![
                content_view,
                setFrame:contect_rect
            ]
        };

        self.panes.render(panes)
    }

    pub fn set_title<T: AsRef<str>>(&self, title: T) {
        unsafe {
            let title = NSString::alloc(nil).init_str(title.as_ref());
            self.window.setTitle_(title);
        }
    }
}
