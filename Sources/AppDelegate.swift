import AppKit
import PathKit
import ReactiveSwift

class AppDelegate: NSObject, NSApplicationDelegate {
    var workspaces = [Workspace]()

    func applicationDidFinishLaunching(_ notification: Notification) {
        print("Did launch!")

        let defaultProject = Project(Path.current)
        let defaultWorkspace = Workspace(defaultProject)
        defaultWorkspace.createBlankWindow()
        workspaces.append(defaultWorkspace)

        print("Created window.")
    }

    func applicationShouldTerminate(_ sender: NSApplication) -> NSApplicationTerminateReply {
        print("Should terminate!")
        return .terminateNow
    }

    func applicationWillTerminate(_ notification: Notification) {
        print("Will terminate!")
    }
}
