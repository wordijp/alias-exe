use std::io;
use std::process::{Child, Command};
use std::io::{Error, ErrorKind};

use crate::lib::encode;

pub fn command_output(cmd: &str) -> io::Result<String> {
    let output = Command::new("cmd")
        .arg("/c")
        .arg(cmd)
        .output()?;

    if !output.status.success() {
        return Err(Error::new(ErrorKind::InvalidData, encode::to_utf8_string(&output.stderr)));
    }

    Ok(encode::to_utf8_string(&output.stdout).trim().to_string())
}

pub fn command_spawn(cmd: &str) -> io::Result<i32> {
    let mut cmd: Child = Command::new("cmd")
        .arg("/c")
        .arg(cmd)
        .spawn()?;
    let status = cmd.wait()?;
    
    Ok(status.code().unwrap())
}
