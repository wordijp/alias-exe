use std::{env, process, fs, io};

use clap::{
    self,
    App, Arg, SubCommand,
    crate_name, crate_authors, crate_version
};

use crate::path;

pub fn run(args: &Vec<String>)  {
    let matches = clap_matches(args);
    if args.len() <= 1 {
        println!("{}", matches.usage());
        return Ok(());
    }

    if let Some(ref matches) = matches.subcommand_matches("edit") {
        edit(matches);
    }
    if let Some(ref _matches) = matches.subcommand_matches("list") {
        let _ = list();
    }
}

fn edit(matches: &clap::ArgMatches<'static>) {
    // TODO: edit alias
    println!("edit!");
    if let Some(name) = matches.value_of("alias_name") {
        let name = name.to_string();
    }
}

fn list() -> io::Result<()> {
    const LISTDIR: &str = "./list";
    for entry in fs::read_dir(LISTDIR)? {
        let path = entry?.path();
        
        if path::is_exe(&path) {
            let name = path.file_stem().unwrap().to_str().unwrap();
            let alias = fs::read_to_string(format!("{}/{}.txt", LISTDIR, name));
            let alias = if alias.is_ok() { alias.unwrap() } else { "".to_owned() }; 
            println!("{}={}", name, alias.trim());
        }
    }

    Ok(())
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
