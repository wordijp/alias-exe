use std::{fs, env};
use std::io::{self, Error, ErrorKind};

use regex::Regex;

use crate::lib::repl;
use crate::lib::cmd;
use crate::lib::term;

pub fn read(listdir: &str, alias_name: &str) -> io::Result<String> {
    fs::read_to_string(format!("{}/{}.txt", listdir, alias_name))
}

enum Alias<'a> {
    SetEnv(&'a str, &'a str),
    Cmd(String),
}

pub fn run(alias_value: &str, args: &Vec<String>) -> io::Result<i32> {
    for value_line in parse_alias_value(alias_value, args)? {
        match parse_alias_type(&value_line)? {
            Alias::SetEnv(key, value) => env::set_var(key, value),
            Alias::Cmd(value) => cmd::command_spawn(&value)?,
        }
    }

    Ok(0)
}

fn parse_alias_value(alias_value: &str, args: &Vec<String>) -> io::Result<Vec<String>> {
    // parse args($1, $2, etc)
    lazy_static! {
        static ref RE_ARGS: Regex = Regex::new(r#"(\$[0-9*@#]|"\$[*@]")"#).unwrap();
    }
    let repl = repl::replace_all_func(&RE_ARGS, alias_value, |caps| parse_arg(caps.get(0).unwrap().as_str(), args))?;
    // replace multiple line
    let repl = repl.replace("^\n", "");
    // split multiple command
    let repls: Vec<String> = repl
        .split('\n')
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(|x| x.to_string())
        .collect();

    Ok(repls)
}

fn parse_alias_type(alias_value: &str) -> io::Result<Alias> {
    lazy_static! {
        static ref RE_PRE_ENV: Regex = Regex::new(r"^@set").unwrap();
        static ref RE_ENV: Regex = Regex::new(r"^@set\s+([^=]+)=(.*)").unwrap();

        static ref RE_NESTED_CMD: Regex = Regex::new(r"\$\((.*?)\)").unwrap();
    }

    if RE_PRE_ENV.is_match(alias_value) {
        let caps = RE_ENV.captures(alias_value);
        if caps.is_none() {
            let (s1, s2, s3) = repl::partition_re(&RE_PRE_ENV, alias_value).unwrap();
            return Err(Error::new(ErrorKind::InvalidData, format!("{}: illegal @set format\n{}{}{}", term::ewrite("failed")?, s1, term::ewrite(s2)?, s3)));
        }

        let caps = caps.unwrap();
        let key = caps.get(1).unwrap().as_str();
        let value = caps.get(2).unwrap().as_str();

        Ok(Alias::SetEnv(key, value))
    } else {
        let value = repl::replace_all_func_nested(&RE_NESTED_CMD, alias_value,
            |caps| cmd::command_output(caps.get(1).unwrap().as_str()))?;
        Ok(Alias::Cmd(value))
    }
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
        "$*" => Err(Error::new(ErrorKind::InvalidData, format!("{}: $* is not supported, maybe \"$*\" ?", term::ewrite("failed")?))),
        "$@" => Err(Error::new(ErrorKind::InvalidData, format!("{}: $@ is not supported, maybe \"$@\" ?", term::ewrite("failed")?))),
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
