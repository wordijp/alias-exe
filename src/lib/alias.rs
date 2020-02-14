use std::{env, fs, path::Path};
use std::io::{self, Error, ErrorKind};
use std::process::{Child, Command};

use crate::lib::path;
use crate::lib::encode;

const LISTDIR: &str = "./list";

// -----

pub struct AliasListIterator {
    dir: io::Result<fs::ReadDir>
}

impl Iterator for AliasListIterator {
    type Item = (String, String);

    fn next(&mut self) -> Option<(String, String)> {
        if let Ok(ref mut dir) = self.dir {
            while let Some(entry) = dir.next() {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path::is_exe(&path) {
                            let name = path.file_stem().unwrap().to_str().unwrap();
                            let alias = fs::read_to_string(format!("{}/{}.txt", LISTDIR, name));
                            let alias = if alias.is_ok() { alias.unwrap() } else { "".to_owned() };
                            return Some((name.to_string(), alias.trim().to_string()));
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
    let alias_path = Path::new(&alias_txt);
    if !alias_path.exists() {
        fs::File::create(&alias_txt)?;
    }

    if try_edit("vim", &alias_txt).is_ok() {
        return Ok(())
    }

    try_edit("notepad", &alias_txt)
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
        return Err(Error::new(ErrorKind::Other, "edit failed"));
    }

    Ok(())
}

// -----

pub fn mklink(alias_name: &str) -> io::Result<()> {
    let alias_exe = format!("{}/{}.exe", LISTDIR, alias_name);
    let alias_path = Path::new(&alias_exe);
    if !alias_path.exists() {
        let current_exe = env::current_exe().unwrap();
        let current_path = Path::new(&current_exe);

        env::set_current_dir(&format!("{}/{}", current_path.parent().unwrap().to_str().unwrap(), LISTDIR)).unwrap();

        // NOTE: mklink path separator is '\'
        let link = format!("{}.exe", alias_name);
        let target = format!("..\\{}", current_path.file_name().unwrap().to_str().unwrap());
        try_mklink(&link, &target)?;
    }

    Ok(())
}
pub fn try_mklink(link: &str, target: &str) -> io::Result<()> {
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
