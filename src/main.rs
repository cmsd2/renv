extern crate clap;

use clap::{Arg, ArgMatches, App, SubCommand};
use std::io::Write;

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
        .get_matches();

    match run_subcommand(&matches) {
        Err(err) => {
            writeln!(&mut stderr, "Error: {:?}", err).unwrap();
        },
        _ => {}
    }
}

fn run_subcommand(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("list", Some(sub_matches)) => list_envs(),
        _ => Ok(())
    }
}

fn list_envs() -> Result<()> {
    try!(init_dirs());
    
    for e in try!(get_envs_list()) {
        println!("{}", e);
    }

    Ok(())
}

fn get_envs_list() -> Result<Vec<String>> {
    let env_list = try!(os::list_env_files());
    
    Ok(env_list)
}

fn init_dirs() -> Result<()> {
    try!(os::get_config_dir());
    
    Ok(())
}
