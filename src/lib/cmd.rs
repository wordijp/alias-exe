use std::io;
use std::process::{Child, Command};
use std::io::{Error, ErrorKind};

use crate::lib::encode;
use crate::lib::term;

pub fn command_output(cmd: &str) -> io::Result<String> {
    let args = split_args(cmd);

    let output = Command::new("cmd")
        .arg("/c")
        .args(args)
        .output()?;

    if !output.status.success() {
        return Err(Error::new(ErrorKind::InvalidData, encode::to_utf8_string(&output.stderr)));
    }
    Ok(encode::to_utf8_string(&output.stdout).trim().to_string())
}

pub fn command_spawn(cmd: &str) -> io::Result<()> {
    let args = split_args(cmd);

    let mut cmd: Child = Command::new("cmd")
        .arg("/c")
        .args(args)
        .spawn()?;
    let status = cmd.wait()?;

    if !status.success() {
        return Err(Error::new(ErrorKind::Interrupted, format!("{}: {}", term::ewrite("failed")?, status.to_string())));
    }
    Ok(())
}

fn split_args(cmd: &str) -> Vec<String> {
    let mut args = Vec::new();

    let mut s = String::new();
    let mut escape = false;
    let mut double_quote = false;

    for ch in cmd.split("") {
        match ch {
            " " => {
                if double_quote {
                    s.push_str(ch);
                } else if s.len() > 0 {
                    args.push(s.clone());
                    s.clear();
                }
            },
            "\\" => {
                if escape {
                    s.push_str("\\");
                    escape = false;
                } else {
                    escape = true;
                }
            },
            "\"" => {
                if escape {
                    s.push_str("\"");
                    escape = false;
                } else {
                    double_quote = !double_quote;
                }
            },
            _ => s.push_str(ch),
        }
    }

    if s.len() > 0 {
        args.push(s);
    }

    args
}
