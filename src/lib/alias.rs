use std::{env, fs, path::Path};
use std::io::{self, Read, Error, ErrorKind};
use std::process::{Child, Command};

use crate::lib::path;
use crate::lib::encode;

const LISTDIR: &str = "./list";

// -----

pub struct AliasListIterator {
    dir: io::Result<fs::ReadDir>
}

impl Iterator for AliasListIterator {
    // type Item = (<alias_name>, <value>)
    type Item = (String, String);

    fn next(&mut self) -> Option<(String, String)> {
        if let Ok(ref mut dir) = self.dir {
            while let Some(entry) = dir.next() {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path::is_exe(&path) {
                            let alias_name = path.file_stem()
                                .and_then(|x| x.to_str())
                                .map(|x| x.to_string())
                                .unwrap();
                            let value = fs::read_to_string(format!("{}/{}.txt", LISTDIR, alias_name))
                                .map(|x| x.trim().to_string())
                                .unwrap_or("".to_owned());
                            return Some((alias_name, value));
                        }
                    },
                    Err(_) => break,
                }
            }
        }

        None
    }
}

pub fn iter() -> AliasListIterator {
    AliasListIterator { dir: fs::read_dir(LISTDIR) }
}

// -----

pub fn edit(alias_name: &str) -> io::Result<()> {
    if !Path::new(LISTDIR).exists() {
        fs::create_dir(LISTDIR)?;
    }

    let alias_txt = format!("{}/{}.txt", LISTDIR, alias_name);
    if !Path::new(&alias_txt).exists() {
        fs::File::create(&alias_txt)?;
    }

    try_edit("vim", &alias_txt)
        .or_else(|_| try_edit("notepad", &alias_txt))
}
fn try_edit(editor: &str, alias_txt: &str) -> io::Result<()> {
    let cmd = Command::new(editor)
        .arg(alias_txt)
        .spawn();
    if let Err(err) = cmd {
        return Err(Error::new(ErrorKind::NotFound, format!("Not found {}\n{}", editor, err)));
    }

    let mut cmd: Child = cmd.unwrap();

    let status = cmd.wait()?;
    if status.code().unwrap() != 0 {
        let mut buf = Vec::new();
        cmd.stderr.unwrap().read_to_end(&mut buf).unwrap();
        return Err(Error::new(ErrorKind::Other, encode::to_utf8_string(&buf)));
    }

    Ok(())
}

// -----

pub fn mklink(alias_name: &str) -> io::Result<()> {
    let alias_exe = format!("{}/{}.exe", LISTDIR, alias_name);
    if Path::new(&alias_exe).exists() {
        return Ok(());
    }

    let current_exe = env::current_exe().unwrap();
    let cwd_bak = env::current_dir().unwrap();

    env::set_current_dir(&format!("{}/{}", current_exe.parent().unwrap().display(), LISTDIR)).expect("failed: change current dir");
    // NOTE: mklink path separator is '\'
    let link = format!("{}.exe", alias_name);
    let target = format!("..\\{}", current_exe.file_name().unwrap().to_str().unwrap());
    let ret = try_mklink(&link, &target);
    env::set_current_dir(cwd_bak).expect("failed: restore current dir");

    ret
}
fn try_mklink(link: &str, target: &str) -> io::Result<()> {
    let output = Command::new("cmd")
        .arg("/c")
        .arg("mklink")
        .arg(link)
        .arg(target)
        .output()?;

    if !output.status.success() {
        return Err(Error::new(ErrorKind::Other, encode::to_utf8_string(output.stderr.as_ref())));
    }

    Ok(())
}

// -----

pub fn remove(alias_name: &str) -> io::Result<()> {
    let alias_exe = format!("{}/{}.exe", LISTDIR, alias_name);
    if !Path::new(&alias_exe).exists() {
        return Err(Error::new(ErrorKind::NotFound, format!("failed: {}.exe is not found", alias_name)));
    }

    fs::remove_file(&alias_exe)?;

    let alias_txt = format!("{}/{}.txt", LISTDIR, alias_name);
    if Path::new(&alias_txt).exists() {
        fs::remove_file(&alias_txt)?;
    }

    Ok(())
}
