
#[cfg(windows)] pub mod win;
#[cfg(windows)] pub use ::os::win::*;

#[cfg(unix)] pub mod unix;
#[cfg(unix)] pub use ::os::unix::*;

use std::env;
use std::ffi::OsString;

pub fn get_editor() -> OsString {
    env::var_os("EDITOR")
        .unwrap_or_else(get_default_editor)
}
