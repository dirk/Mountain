import AppKit
import PathKit

class File {
    let path: Path

    init(_ withPath: Path) {
        path = withPath
    }
}

func ==(lhs: File, rhs: File) -> Bool {
    return lhs.path == rhs.path
}

// Where a buffer came from. This is used when one tries to save the buffer.
enum BufferSource {
    case file(File)
    case unknown
}

func ==(lhs: BufferSource, rhs: BufferSource) -> Bool {
    switch (lhs, rhs) {
    case (.file(let lf), .file(let rf)) where lf == rf:
        return true
    case (.unknown, .unknown):
        return true
    default:
        return false
    }
}

class Line {
    var characters: String

    init(_ withCharacters: String) {
        characters = withCharacters
    }
}

class Buffer {
    var source: BufferSource
    var lines = [Line]()

    init() {
        source = .unknown
    }
}

func ==(lhs: Buffer, rhs: Buffer) -> Bool {
    return lhs.source == rhs.source
}
