use std::io;
use std::fs;
use std::path::{Path,PathBuf};

pub fn make_dir(path: &Path) -> io::Result<()> {
    let err = match fs::create_dir(path) {
        Ok(_) => return Ok(()),
        Err(e) => e
    };

    match err.kind() {
        io::ErrorKind::AlreadyExists => return Ok(()),
        _ => {
            return Err(err);
        }
    };
}

pub fn make_dirs(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        try!(make_dirs(parent));
    }

    if !path.exists() {
        try!(make_dir(path));
    }

    Ok(())
}

pub fn get_installed_env_file(env_name: &str) -> ::result::Result<PathBuf> {
    let mut env_dir = try!(::os::get_config_dir());
    let env_file_name = format!("{}.env", env_name);

    env_dir.push(env_file_name);

    Ok(env_dir)
}
