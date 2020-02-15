use std::fs;
use std::io::{self, Error, ErrorKind};
use std::process::{Child, Command};

use regex::Regex;

use crate::lib::repl;

pub fn read(listdir: &str, alias_name: &str) -> io::Result<String> {
    fs::read_to_string(format!("{}/{}.txt", listdir, alias_name))
}

pub fn run(alias_value: &str, args: &Vec<String>) -> io::Result<i32> {
    let parsed_alias_value = parse_alias_value(alias_value, args)?;

    let mut cmd: Child = Command::new("cmd")
        .arg("/c")
        .arg(parsed_alias_value)
        .spawn()?;

    let status = cmd.wait()?;
    Ok(status.code().unwrap())
}

fn parse_alias_value(alias_value: &str, args: &Vec<String>) -> io::Result<String> {
    lazy_static! {
        static ref RE_ARGS: Regex = Regex::new(r#"(\$[0-9*@#]|"\$[*@]")"#).unwrap();
    }
    let repl = repl::replace_all_func(&RE_ARGS, alias_value, |x| parse_arg(x, args))?;

    Ok(repl)
}

fn parse_arg(arg: &str, args: &Vec<String>) -> io::Result<String> {
    match arg {
        "$0" => Ok(args[0].clone()),
        "$1" => Ok(if args.len() > 1 { args[1].clone() } else { "".to_owned() }),
        "$2" => Ok(if args.len() > 2 { args[2].clone() } else { "".to_owned() }),
        "$3" => Ok(if args.len() > 3 { args[3].clone() } else { "".to_owned() }),
        "$4" => Ok(if args.len() > 4 { args[4].clone() } else { "".to_owned() }),
        "$5" => Ok(if args.len() > 5 { args[5].clone() } else { "".to_owned() }),
        "$6" => Ok(if args.len() > 6 { args[6].clone() } else { "".to_owned() }),
        "$7" => Ok(if args.len() > 7 { args[7].clone() } else { "".to_owned() }),
        "$8" => Ok(if args.len() > 8 { args[8].clone() } else { "".to_owned() }),
        "$9" => Ok(if args.len() > 9 { args[9].clone() } else { "".to_owned() }),
        "$#" => Ok(format!("{}", args.len() - 1)),
        "$*" => Err(Error::new(ErrorKind::InvalidData, "failed: $* is not suppurted, maybe \"$*\" ?")),
        "$@" => Err(Error::new(ErrorKind::InvalidData, "failed: $@ is not suppurted, maybe \"$@\" ?")),
        "\"$*\"" => Ok(str_join(args.iter().skip(1), " ")),
        "\"$@\"" => Ok(str_join(args.iter().skip(1), " ")),
        _ => Ok(arg.to_string()),
    }
}

fn str_join<'a, I>(mut it: I, sep: &str) -> String
where
    I: Iterator<Item = &'a String>
{
    let mut s = String::new();
    if let Some(first) = it.nth(0) {
        s.push_str(first);
    }
    for x in it {
        s.push_str(sep);
        s.push_str(x);
    }
    s
}
