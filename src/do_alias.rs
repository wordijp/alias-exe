use std::{env, process, io};

use clap::{
    self,
    App, Arg, SubCommand,
    crate_name, crate_authors, crate_version
};

use crate::lib;
use crate::lib::term;

pub fn run(args: &Vec<String>) {
    // current directory to exe dir
    let current_exe = env::current_exe().unwrap();
    env::set_current_dir(current_exe.parent().unwrap()).expect(&format!("{}: change current dir", term::ewrite("failed").unwrap()));

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
    if let Some(ref matches) = matches.subcommand_matches("remove") {
        remove(matches)?;
    }
    if let Some(ref matches) = matches.subcommand_matches("list") {
        list(matches)?;
    }
    if let Some(ref _matches) = matches.subcommand_matches("repair") {
        repair()?;
    }

    Ok(())
}

fn edit(matches: &clap::ArgMatches<'static>) -> io::Result<()> {
    if let Some(alias_name) = matches.value_of("alias_name") {
        lib::alias::validate(alias_name)?;
        lib::alias::edit(alias_name)?;
        lib::alias::mklink(alias_name)?;
    }
    Ok(())
}

fn remove(matches: &clap::ArgMatches<'static>) -> io::Result<()> {
    if let Some(alias_name) = matches.value_of("alias_name") {
        lib::alias::remove(alias_name)?;
        println!("{} removed", term::keywrite(alias_name)?);
    }
    Ok(())
}

fn list(matches: &clap::ArgMatches<'static>) -> io::Result<()> {
    if matches.is_present("key") {
        for (key, _value) in lib::alias::list_iter()? {
            println!("{}", term::keywrite(&key)?);
        }
    } else {
        for (key, value) in lib::alias::list_iter()? {
            print!("{}", term::keywrite(&key)?);
            println!(":\n{}", value);
        }
    }
    Ok(())
}

fn repair() -> io::Result<()> {
    for (key, _value) in lib::alias::cfg_iter()? {
        lib::alias::mklink(&key)?;
        println!("{} repaired", term::keywrite(&key)?);
    }

    Ok(())
}

// ---

const USAGE: &str = "\
    alias <SUBCOMMAND>

SUBCOMMAND:
    help    Prints help information
    edit    Edit alias, new or existing
    remove  Remove alias
    list    List aliases
    repair  Repair aliases from .txt";

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
        .subcommand(
            SubCommand::with_name("remove")
                .arg(Arg::from_usage("<alias_name> 'alias exe name'"))
        )
        .subcommand(
            SubCommand::with_name("list")
                .arg(Arg::from_usage("-k --key 'List up only key'"))
        )
        .subcommand(
            SubCommand::with_name("repair")
        )
        .get_matches_from(args)
}
