#[cfg(target_os = "macos")]
pub use platform::macos::{Application};

#[cfg(target_os = "macos")]
mod macos;
