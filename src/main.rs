extern crate clap;

use clap::{Arg, ArgMatches, App, SubCommand};
use std::io::Write;
use std::process;

use result::*;

pub mod result;
pub mod fs;
pub mod os;

fn main() {
    let mut stderr = std::io::stderr();

    let matches = App::new("rsenv")
        .version("1.0")
        .author("Chris Dawes <cmsd2@cantab.net>")
        .about("Manages shell environments")
        .subcommand(SubCommand::with_name("list")
                    .about("Lists available shell environments")
                    .author("Chris Dawes <cmsd2@cantab.net>")
                    .version("1.0")
                    )
        .subcommand(SubCommand::with_name("remove")
                    .about("Removes installed environment from global environments dir")
                    .author("Chris Dawes <cmsd2@cantab.net>")
                    .version("1.0")
                    .arg(Arg::with_name("name")
                         .short("n")
                         .required(true)
                         .takes_value(true)
                         .index(1)
                         .help("name of environment to remove")
                         )
                    )
        .subcommand(SubCommand::with_name("install")
                    .about("Installs env file in global environments dir")
                    .author("Chris Dawes <cmsd2@cantab.net>")
                    .version("1.0")
                    .arg(Arg::with_name("name")
                         .short("n")
                         .required(true)
                         .takes_value(true)
                         .index(1)
                         .help("name of environment being installed")
                         )
                    .arg(Arg::with_name("file")
                         .short("f")
                         .required(true)
                         .takes_value(true)
                         .index(2)
                         .help("the environment file to install")
                         )
                    )
        .subcommand(SubCommand::with_name("exec")
                    .about("Exec a shell command inside an environment")
                    .author("Chris Dawes <cmsd2@cantab.net>")
                    .version("1.0")
                    .arg(Arg::with_name("name")
                         .short("n")
                         .required(true)
                         .takes_value(true)
                         .index(1)
                         .help("name of environment to load")
                         )
                    .arg(Arg::with_name("command")
                         .short("c")
                         .required(true)
                         .takes_value(true)
                         .index(2)
                         .multiple(true)
                         .help("shell command to run")
                         )
                    )
        .get_matches();

    match run_subcommand(&matches) {
        Err(err) => {
            writeln!(&mut stderr, "Error: {:?}", err).unwrap();
        },
        _ => {}
    }
}

fn run_subcommand(matches: &ArgMatches) -> Result<i32> {
    match matches.subcommand() {
        ("list", Some(_)) => {
            try!(list_envs());
            Ok(0)
        }
        ("install", Some(sub_matches)) => {
            try!(install_env(sub_matches));
            Ok(0)
        },
        ("remove", Some(sub_matches)) => {
            try!(remove_env(sub_matches));
            Ok(0)
        },
        ("exec", Some(sub_matches)) => exec_command(sub_matches),
        _ => Ok(0)
    }
}

fn list_envs() -> Result<()> {
    try!(init_dirs());
    
    for e in try!(get_envs_list()) {
        println!("{}", e);
    }

    Ok(())
}

fn install_env(args: &ArgMatches) -> Result<()> {
    let file_name = args.value_of("file").unwrap();
    let env_name = args.value_of("name").unwrap();

    let file_path = std::path::Path::new(file_name);
    try!(assert_file_exists(&file_path));
    
    let dest_file_path = try!(fs::get_installed_env_file(env_name));

    try!(std::fs::copy(file_path, dest_file_path.as_path()));

    Ok(())
}

fn remove_env(args: &ArgMatches) -> Result<()> {
    let env_name = args.value_of("name").unwrap();

    let dest_file_path = try!(fs::get_installed_env_file(env_name));
    try!(assert_file_exists(&dest_file_path));

    try!(std::fs::remove_file(dest_file_path));

    Ok(())
}

fn exec_command(args: &ArgMatches) -> Result<i32> {
    let command_line: Vec<&str> = args.values_of("command").unwrap().collect();

    let mut command_line_iter = command_line.into_iter();
    
    let command_name = command_line_iter.next().unwrap();
    
    let args: Vec<&str> = command_line_iter.collect();

    {
        let mut result = try!(process::Command::new(command_name)
            .args(&args)
            .spawn());

        let status = try!(result.wait());

        status.code().ok_or(Error::Killed)
    }   
}

fn assert_file_exists(file_path: &std::path::Path) -> Result<()> {
    if !file_path.exists() {
        Err(Error::FileNotFound(file_path.to_string_lossy().into_owned()))
    } else {
        Ok(())
    }
}

fn file_name_to_env_name<'a>(file_name: &'a str) -> Option<&'a str> {
    let split: Vec<&'a str> = file_name
        .rsplitn(2, ".env")
        .collect();

    if split.len() == 2 {
        split.get(1).map(|o| *o)
    } else {
        None
    }
}

fn get_envs_list() -> Result<Vec<String>> {
    let env_file_list = try!(os::list_env_files());

    let maybe_env_list = env_file_list
        .iter()
        .flat_map(|file_name| file_name_to_env_name(file_name))
        .map(|s| s.to_owned())
        .collect();
    
    Ok(maybe_env_list)
}

fn init_dirs() -> Result<()> {
    try!(os::get_config_dir());
    
    Ok(())
}
