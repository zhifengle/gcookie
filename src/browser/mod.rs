pub mod cookie;
pub mod utils;

mod rookie_utils;
pub use rookie_utils::*;

#[cfg(target_os = "windows")]
mod chromium_windows;
#[cfg(target_os = "windows")]
pub use chromium_windows::*;

mod firefox;

#[deprecated(since = "0.1.0", note = "This module is deprecated and will be removed in future versions.")]
pub mod gcookie_utils;
