use std::io;
use std::fs;
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use std::process;
use std::ffi::OsStr;
use std::env;
use ::result::*;
use ::os::*;
use java_properties::*;

#[derive(Debug,Clone,PartialEq)]
pub struct RsEnv {
    pub vars: HashMap<String,String>,
}

impl RsEnv {
    pub fn new_empty() -> RsEnv {
        RsEnv {
            vars: HashMap::new(),
        }
    }
    
    pub fn new(vars: HashMap<String, String>) -> RsEnv {
        RsEnv {
            vars: vars,
        }
    }
}

pub fn get_home_dir() -> Result<PathBuf> {
    match env::home_dir() {
        Some(p) => Ok(p),
        None => Err(RsEnvError::Error("couldn't get your home directory".to_owned()))
    }
}

pub fn list_env_files() -> Result<Vec<String>> {
    let envs_dir = try!(get_config_dir());

    let mut result = vec![];
    
    for env_file in try!(envs_dir.read_dir()) {
        let name = try!(env_file);

        result.push(try!(name.file_name().to_str().ok_or(RsEnvError::StringError)).to_owned());
    }
    
    Ok(result)
}

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

pub fn get_installed_env_file(env_name: &str) -> Result<PathBuf> {
    let mut env_dir = try!(::os::get_config_dir());
    let env_file_name = format!("{}.env", env_name);

    env_dir.push(env_file_name);

    Ok(env_dir)
}

pub fn load_installed_env_file(env_name: &str) -> Result<RsEnv> {
    let env_file_path = try!(get_installed_env_file(env_name));
    try!(assert_file_exists(&env_file_path));

    let file = try!(fs::File::open(env_file_path));

    let mut vars_map = HashMap::new();
    try!(PropertiesIter::new(io::BufReader::new(file)).read_into(|k, v| {
        vars_map.insert(k, v);
    }));

    Ok(RsEnv::new(vars_map))
}

pub fn edit_installed_env_file(env_name: &str) -> Result<()> {
    let env_file_path = try!(get_installed_env_file(env_name));
    try!(assert_file_exists(&env_file_path));

    let editor = get_editor();
    let args = vec![env_file_path];
    let env = RsEnv::new_empty();

    spawn_command(editor.as_os_str(), &args[..], &env)
}

pub fn assert_file_exists(file_path: &Path) -> Result<()> {
    if !file_path.exists() {
        Err(RsEnvError::FileNotFound(file_path.to_string_lossy().into_owned()))
    } else {
        Ok(())
    }
}

pub fn spawn_command<S>(command_str: &OsStr, args: &[S], env: &RsEnv) -> Result<()> where S: AsRef<OsStr> {
        
    let mut command = process::Command::new(command_str);
    command.args(&args);

    for (k,v) in &env.vars {
        command.env(k, v);
    }

    {
        let mut result = try!(command.spawn());
        
        let status = try!(result.wait());
        
        status.code().ok_or(RsEnvError::Killed).and_then(|code| {
            if code == 0 {
                Ok(())
            } else {
                Err(RsEnvError::ChildExited(code))
            }
        })
    }
}
