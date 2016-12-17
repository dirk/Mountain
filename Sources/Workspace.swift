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

// Top-level context instance. Runs similar to a React application, but
// manages its own updating/drawing rather than using the virtual DOM.
// Owns two things:
//
//   State:
//     - Project
//     - Panes
//   Interface:
//     Live application interface (view controllers, NSView's, etc.) that are
//     `render()`'ed when state changes.
class Workspace {
    // Project is immutable. You can't change the project a workspace is
    // associated with. Instead you should just open up a new workspace.
    let project: Project
    // Array of open panes. This is a primitive array because it's high-level
    // state bookkeeping. The potentially-complex arrangement of panes is
    // owned by the interface.
    var panes = [Pane]()
    // Must be a member of `panes`.
    var activePane: Pane?

    var window: Window? = nil

    init(_ withProject: Project) {
        project = withProject
    }

    func createBlankWindow() {
        window = Window(self)
        window!.render(panes: panes)
    }
}
