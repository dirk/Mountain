import AppKit

class File {}

// Where a buffer came from. This is used when one tries to save the buffer.
enum BufferSource {
    case file(File)
    case unknown
}

class Line {
    var characters: String

    init(_ withCharacters: String) {
        characters = withCharacters
    }
}

class Buffer {
    var file: File?
    var lines = [Line]()
}
