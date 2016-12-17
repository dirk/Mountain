import AppKit

let application = NSApplication.shared()
application.setActivationPolicy(.regular)

let delegate = AppDelegate()
application.delegate = delegate
application.activate(ignoringOtherApps: true)
application.run()
