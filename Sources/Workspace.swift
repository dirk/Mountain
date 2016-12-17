import AppKit
import PathKit

class Project {
    let directory: Path

    init(_ withPath: Path) {
        directory = withPath
    }

    init(_ withString: String) {
        directory = Path(withString)
    }
}

class Window {
    let window: NSWindow
    var workspace: Workspace

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
}

class Workspace {
    let project: Project

    var window: Window? = nil

    init(_ withProject: Project) {
        project = withProject
    }

    func createBlankWindow() {
        window = Window(self)
    }
}
