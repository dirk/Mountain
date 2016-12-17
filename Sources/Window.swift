import AppKit

class Window {
    var workspace: Workspace

    let window: NSWindow
    var viewController: WindowContentViewController? = nil

    let panes = [Pane]()

    init(_ withWorkspace: Workspace) {
        workspace = withWorkspace

        window = NSWindow(
            contentRect: NSMakeRect(100, 100, 600, 200),
            styleMask: [.titled, .closable, .miniaturizable, .resizable],
            backing: .buffered,
            defer: false)

        window.cascadeTopLeft(from: NSPoint(x: 100, y: 100))
        window.makeKeyAndOrderFront(nil)
    }

    func render() {
        if viewController == nil {
            viewController = WindowContentViewController(self)
            window.contentView = viewController!.view
        }

        viewController!.render(panes: panes)
    }
}
