import AppKit

class Tab {
    var buffer: Buffer

    init(_ withBuffer: Buffer) {
      buffer = withBuffer
    }
}

func ==(lhs: Tab, rhs: Tab) -> Bool {
    return lhs.buffer == rhs.buffer
}

// A pane is a section of an editor window dedicated to editing a buffer. It
// can have multiple tabs.
class Pane {
    var tabs = [Tab]()
}

func ==(lhs: Pane, rhs: Pane) -> Bool {
    if lhs.tabs.count == rhs.tabs.count {
        for (i, lt) in lhs.tabs.enumerated() {
            if !(lt == rhs.tabs[i]) {
                return false
            }
        }
        return true
    } else {
        return false
    }
}
