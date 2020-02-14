use std::{env, process, io};
use std::path::Path;

use clap::{
    self,
    App, Arg, SubCommand,
    crate_name, crate_authors, crate_version
};

use crate::lib;

pub fn run(args: &Vec<String>) {
    // current directory to exe dir
    let cwd = env::current_exe().unwrap();
    let path = Path::new(&cwd);
    env::set_current_dir(path.parent().unwrap()).expect("failed: change current dir");

    if let Err(err) = try_run(args) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn try_run(args: &Vec<String>) -> io::Result<()> {
    let matches = clap_matches(args);
    if args.len() <= 1 {
        println!("{}", matches.usage());
        return Ok(());
    }

    if let Some(ref matches) = matches.subcommand_matches("edit") {
        edit(matches)?;
    }
    if let Some(ref _matches) = matches.subcommand_matches("list") {
        list();
    }

    Ok(())
}

fn edit(matches: &clap::ArgMatches<'static>) -> io::Result<()> {
    if let Some(name) = matches.value_of("alias_name") {
        lib::alias::edit(name)?;
        lib::alias::mklink(name)?;
    }
    Ok(())
}

fn list() {
    for (key, value) in lib::alias::iter() {
        println!("{}={}", key, value);
    }
}

// ---

const USAGE: &str = "\
    alias <SUBCOMMAND>

SUBCOMMAND:
    help    Prints help information
    edit    Edit alias, new or existing
    list    List aliases";

const TEMPLATE: &str = "\
{bin} {version}
{author}

USAGE:
    {usage}

FLAGS:
{unified}";

fn clap_matches(args: &Vec<String>) -> clap::ArgMatches<'static> {
    App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .usage(USAGE)
        .template(TEMPLATE)
        .subcommand(
            SubCommand::with_name("edit")
                .arg(Arg::from_usage("<alias_name> 'alias exe name'"))
        )
        .subcommand(SubCommand::with_name("list"))
        .get_matches_from(args)
}
