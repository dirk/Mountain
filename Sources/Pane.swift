import AppKit

class PaneItem {
    var buffer: Buffer

    init(_ withBuffer: Buffer) {
      buffer = withBuffer
    }
}

class Pane {
    var items = [PaneItem]()
}
