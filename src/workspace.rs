use pane::Pane;

struct Project;

struct Window;

struct Workspace {
    project: Project,
    /// Window to which the `Workspace` is rendered.
    window: Window,
    panes: [Pane; 1],
}
