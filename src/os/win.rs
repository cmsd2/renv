
use ::fs;
use ::result::*;
use std::env;
use std::path::{Path,PathBuf};
use std::ffi::OsString;

pub fn get_roaming_dir() -> Result<PathBuf> {
    match env::var_os("APPDATA") {
        Some(os_str) => {
            Ok(Path::new(&os_str).to_path_buf())
        },
        None => {
            Err(RsEnvError::Error("couldn't get appdata dir".to_owned()))
        }
    }
}

pub fn get_config_dir() -> Result<PathBuf> {
    let mut p = try!(get_roaming_dir());

    p.push("rsenv");
    p.push("envs");

    try!(fs::make_dirs(&p));
    
    Ok(p)
}

pub fn get_default_editor() -> OsString {
    OsString::from("notepad".to_owned())
}

