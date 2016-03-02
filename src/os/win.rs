
use ::fs;
use ::result::*;
use std::env;
use std::path::{Path,PathBuf};

pub fn get_home_dir() -> Result<PathBuf> {
    match env::home_dir() {
        Some(p) => Ok(p),
        None => Err(Error::Error("couldn't get your home directory".to_owned()))
    }
}

pub fn get_roaming_dir() -> Result<PathBuf> {
    match env::var_os("APPDATA") {
        Some(os_str) => {
            Ok(Path::new(&os_str).to_path_buf())
        },
        None => {
            Err(Error::Error("couldn't get appdata dir".to_owned()))
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

pub fn list_env_files() -> Result<Vec<String>> {
    let envs_dir = try!(get_config_dir());

    let mut result = vec![];
    
    for env_file in try!(envs_dir.read_dir()) {
        let name = try!(env_file);

        result.push(try!(name.file_name().to_str().ok_or(Error::StringError)).to_owned());
    }
    
    Ok(result)
}
