import AppKit

class WindowContentViewController {
    let window: Window
    let view = NSView()

    var currentPanes = [Pane]()
    var currentPaneViewControllers = [PaneViewController]()

    init(_ withWindow: Window) {
        window = withWindow
    }

    func render(panes: [Pane]) {
        let panesUnchanged = panes.elementsEqual(currentPanes, by: { lhs, rhs in
            return lhs == rhs
        })

        if !panesUnchanged {
            resetPanes(panes)
        }

        for paneViewController in currentPaneViewControllers {
            paneViewController.render()
        }
    }

    private func resetPanes(_ panes: [Pane]) {
        // Clean up old view controllers
        for viewController in currentPaneViewControllers {
            viewController.view.removeFromSuperview()
        }

        currentPanes = panes
        currentPaneViewControllers = panes.map { pane in
            let viewController = PaneViewController(pane)
            self.view.addSubview(viewController.view)
            return viewController
        }
    }
}
