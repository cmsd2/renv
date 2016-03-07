
use ::fs;
use ::result::*;
use std::env;
use std::path::{Path,PathBuf};
use std::ffi::OsString;

pub fn get_xdg_config_dir() -> Result<PathBuf> {
    match env::var_os("HOME") {
        Some(os_str) => {
            let mut dir = Path::new(&os_str).to_path_buf();
            dir.push(".config");
            Ok(dir)
        },
        None => {
            Err(REnvError::Error("couldn't get xdg config dir".to_owned()))
        }
    }
}

pub fn get_config_dir() -> Result<PathBuf> {
    let mut p = try!(get_xdg_config_dir());

    p.push("renv");
    p.push("envs");

    try!(fs::make_dirs(&p));
    
    Ok(p)
}

pub fn get_default_editor() -> OsString {
    OsString::from("nano".to_owned())
}

