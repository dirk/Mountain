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

class Workspace {
    let project: Project

    var window: Window? = nil

    init(_ withProject: Project) {
        project = withProject
    }

    func createBlankWindow() {
        window = Window(self)
        window!.render()
    }
}
