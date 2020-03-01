use std::{fs, env, ops};
use std::io::{self, Error, ErrorKind};

use regex::Regex;
use mrusty::*;

use crate::lib::repl;
use crate::lib::cmd;
use crate::lib::term;
use crate::lib::dsl;

pub fn read(listdir: &str, alias_name: &str) -> io::Result<String> {
    fs::read_to_string(format!("{}/{}.txt", listdir, alias_name))
}

enum Parsed<'a> {
    SetEnv(&'a str, &'a str),
    Cmd(&'a str),
    Mruby(&'a str),
}

pub fn run(alias_value: &str, args: &Vec<String>) -> io::Result<i32> {
    // error, oh... ( help: the trait `std::marker::Sync` is not implemented for `std::rc::Rc<std::cell::RefCell<mrusty::mruby::Mruby>>` )
    //lazy_static! {
    //    static ref MRUBY: mrusty::MrubyType = mruby::mruby_new().unwrap();
    //}
    let mruby = dsl::mruby::mruby_new().unwrap();

    parse_alias_value(alias_value, args, &mruby, |parsed| {
        match parsed {
            Parsed::SetEnv(key, value) => env::set_var(key, value),
            Parsed::Cmd(source) => cmd::command_spawn(source)?,
            Parsed::Mruby(source) => { mruby_run(&mruby, source)?; },
        }
        Ok(())
    })?;

    Ok(0)
}

fn mruby_run(mruby: &mrusty::MrubyType, source: &str) -> io::Result<mrusty::Value> {
    let result = mruby.run(source);
    if let Err(err) = result {
        return Err(Error::new(ErrorKind::InvalidData, format!("{}: {}\n\n{}", term::ewrite("mruby failed")?, err, source)));
    }
    Ok(result.unwrap())
}

fn parse_alias_value(
    alias_value: &str,
    args: &Vec<String>,
    mruby: &mrusty::MrubyType,
    frun: impl Fn(Parsed) -> io::Result<()>
)
    -> io::Result<()>
{
    const NESTED_CMD: &'static str = r"\$\((.*?)\)";
    const NESTED_MRUBY: &'static str = r"<%=(.*?)%>";
    lazy_static! {
        // parse args($1, $2, etc)
        static ref RE_ARGS: Regex = Regex::new(r#"(""\$[*@]""|"\$[*@]"|\$[0-9*@#])"#).unwrap();
        // parse nested $( ... ) or <%= ... %>
        static ref RE_NESTED_CMD: Regex = Regex::new(NESTED_CMD).unwrap();
        static ref RE_NESTED_MRUBY: Regex = Regex::new(NESTED_MRUBY).unwrap();
        static ref RE_NESTED: Regex = Regex::new(&format!("{}|{}", NESTED_CMD, NESTED_MRUBY)).unwrap();
    }
    let alias_value = repl::replace_all_func(&RE_ARGS, alias_value, |caps| parse_arg(caps.get(0).unwrap().as_str(), args))?;

    // replace multiple line for cmd
    let alias_value = alias_value.replace("^\n", "");

    let run_nested = |caps: &regex::Captures| {
        let s = caps.get(0).unwrap().as_str();
        if &s[0..2] == "$(" {
            // nested cmd
            let cap = RE_NESTED_CMD.captures(&s).unwrap();
            let source = cap.get(1).unwrap().as_str();
            cmd::command_output(source)
        } else {
            // nested mruby
            let cap = RE_NESTED_MRUBY.captures(&s).unwrap();
            let source = cap.get(1).unwrap().as_str();
            let value = mruby_run(mruby, source)?;
            dsl::mruby::value2str(mruby, value)
        }
    };

    split_source_func(&alias_value, |source| {
        match source {
            Source::Cmd(cmd_source) => {
                validate_nested(cmd_source)?;
                let cmd_source = repl::replace_all_func_nested(&RE_NESTED, cmd_source, run_nested)?;
                frun(parse_alias_type(&cmd_source)?)?;
            },
            Source::Mruby(mruby_source) => {
                validate_nested(mruby_source)?;
                let mruby_source = repl::replace_all_func_nested(&RE_NESTED, mruby_source, run_nested)?;
                frun(Parsed::Mruby(&mruby_source))?;
            },
        }
        Ok(())
    })?;

    Ok(())
}

// ---

enum Source<'a> {
    Cmd(&'a str),
    Mruby(&'a str),
}

fn split_source_func(
    alias_value: &str,
    fsource: impl Fn(Source) -> io::Result<()>
)
    -> io::Result<()>
{
    lazy_static! {
        static ref RE_MRUBY_RANGE: Regex = Regex::new(r"(?ms)^\s*```ruby\s*$\n(.+?)\n^\s*```\s*$").unwrap();
    }

    let mut cur = 0;
    for caps in RE_MRUBY_RANGE.captures_iter(alias_value) {
        let m = caps.get(0).unwrap();
        let start = m.start();
        if start > cur {
            // cmd source
            for cmd_source in alias_value[cur..start].trim()
                .split('\n')
                .map(|x| x.trim())
                .filter(|x| x.len() > 0)
            {
                fsource(Source::Cmd(cmd_source))?;
            }
        }

        // mruby source
        let m1 = caps.get(1).unwrap();
        let mruby_source = &alias_value[m1.start()..m1.end()].trim();
        fsource(Source::Mruby(mruby_source))?;

        cur = m.end();
    }

    // remain cmd source
    if cur < alias_value.len() {
        for cmd_source in alias_value[cur..].trim()
            .split('\n')
            .map(|x| x.trim())
            .filter(|x| x.len() > 0)
        {
            fsource(Source::Cmd(cmd_source))?;
        }
    }
    Ok(())
}

// ---

fn parse_alias_type(alias_value: &str) -> io::Result<Parsed> {
    lazy_static! {
        static ref RE_PRE_ENV: Regex = Regex::new(r"^@set").unwrap();
        static ref RE_ENV: Regex = Regex::new(r"^@set\s+([^=]+)=(.*)").unwrap();
    }

    if RE_PRE_ENV.is_match(alias_value) {
        let caps = RE_ENV.captures(alias_value);
        if caps.is_none() {
            let (s1, s2, s3) = repl::partition_re(&RE_PRE_ENV, alias_value).unwrap();
            return Err(Error::new(ErrorKind::InvalidData, format!("{}: illegal @set format\n\n{}{}{}", term::ewrite("failed")?, s1, term::ewrite(s2)?, s3)));
        }

        let caps = caps.unwrap();
        let key = caps.get(1).unwrap().as_str();
        let value = caps.get(2).unwrap().as_str();

        Ok(Parsed::SetEnv(key, value))
    } else {
        Ok(Parsed::Cmd(alias_value))
    }
}

fn validate_nested(alias_value: &str) -> io::Result<()> {
    lazy_static! {
        static ref RE_NESTED: Regex = Regex::new(r"(\$\(|<%=|%>|\))").unwrap();
    }

    let mut nested_cmd: Vec<ops::Range<usize>> = Vec::new();
    let mut nested_mruby: Vec<ops::Range<usize>> = Vec::new();
    let mut erred: Vec<ops::Range<usize>> = Vec::new();

    for caps in RE_NESTED.captures_iter(alias_value) {
        let elm = caps.get(0).unwrap();
        match elm.as_str() {
            "$(" => {
                nested_cmd.push(elm.range());
            },
            ")" => {
                let mut ng = false;
                if let Some(cmd_rng) = nested_cmd.pop() {
                    if let Some(mruby_rng) = nested_mruby.last() {
                        if cmd_rng.end < mruby_rng.end {
                            ng = true;
                        }
                    }
                } else {
                    ng = true;
                }

                if ng {
                    erred.push(elm.range());
                }
            },
            "<%=" => {
                nested_mruby.push(elm.range());
            },
            "%>" => {
                let mut ng = false;
                if let Some(mruby_rng) = nested_mruby.pop() {
                    if let Some(cmd_rng) = nested_cmd.last() {
                        if mruby_rng.end < cmd_rng.end {
                            ng = true;
                        }
                    }
                } else {
                    ng = true;
                }

                if ng {
                    erred.push(elm.range());
                }
            }

            _ => {},
        }
    }

    if nested_cmd.len() > 0 || nested_mruby.len() > 0 || erred.len() > 0 {
        erred.append(&mut nested_cmd);
        erred.append(&mut nested_mruby);
        erred.sort_by(|a, b| a.start.cmp(&b.start));

        // Colorize error location
        let mut s = String::new();
        let mut idx = 0;
        for rng in erred {
            s.push_str(&alias_value[idx..rng.start]);
            s.push_str(&term::ewrite(&alias_value[rng.start..rng.end])?);
            idx = rng.end;
        }
        if idx < alias_value.len() {
            s.push_str(&alias_value[idx..]);
        }
        return Err(Error::new(ErrorKind::InvalidData, format!("{}: nested command syntax error\n\n{}", term::ewrite("failed")?, s)));
    }

    Ok(())
}

fn parse_arg(arg: &str, args: &Vec<String>) -> io::Result<String> {
    let f = |s: &str| {
        if let Some(_) = s.find(char::is_whitespace) {
            format!(r#""{}""#, s)
        } else {
            s.to_owned()
        }
    };

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
        "\"$*\"" => Ok(str_join(args.iter().skip(1).map(|x| x.to_string()), " ")),
        "\"$@\"" => Ok(str_join(args.iter().skip(1).map(|x| x.to_string()), " ")),
        "\"\"$*\"\"" => Ok(str_join(args.iter().skip(1).map(|x| f(x)), " ")),
        "\"\"$@\"\"" => Ok(str_join(args.iter().skip(1).map(|x| f(x)), " ")),
        _ => Ok(arg.to_string()),
    }
}

fn str_join<'a, I>(mut it: I, sep: &str) -> String
where
    I: Iterator<Item = String>
{
    let mut s = String::new();
    if let Some(first) = it.nth(0) {
        s.push_str(&first);
    }
    for x in it {
        s.push_str(sep);
        s.push_str(&x);
    }
    s
}
