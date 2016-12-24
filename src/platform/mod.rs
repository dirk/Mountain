pub mod common;

#[cfg(target_os = "macos")]
#[path="macos/mod.rs"]
pub mod local;

pub use self::local::{Application, Window};
