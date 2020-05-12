use std::{env, fs, path};
use std::io::{self, Error, ErrorKind};

use regex::Regex;
use crate::lib::term;

pub const LISTDIR: &str = "./list";

pub fn self_is_symlink() -> bool {
    env::current_exe().unwrap().read_link().is_ok()
}

lazy_static! {
    static ref RE_EXE: Regex = Regex::new(r"\.[eE][xX][eE]$").unwrap();
}

pub fn is_exe(path: &path::PathBuf) -> bool {
    RE_EXE.is_match(path.to_str().unwrap()) && fs::metadata(path).unwrap().is_file()
}


lazy_static! {
    static ref RE_TXT: Regex = Regex::new(r"\.[tT][xX][tT]$").unwrap();
}

pub fn is_txt(path: &path::PathBuf) -> bool {
    RE_TXT.is_match(path.to_str().unwrap()) && fs::metadata(path).unwrap().is_file()
}

pub fn cfg_path() -> io::Result<String> {
    const CFG_DIR: &'static str = ".alias-exe";

    Ok(format!("{}\\{}", home_path()?, CFG_DIR))
}

pub fn cfg_list_path() -> io::Result<String> {
    Ok(format!("{}\\{}", cfg_path()?, LISTDIR))
}


fn home_path() -> io::Result<String> {
    const USERPROFILE: &'static str = "USERPROFILE";
    const HOMEDRIVE: &'static str = "HOMEDRIVE";
    const HOMEPATH: &'static str = "HOMEPATH";

    let home = env::var(USERPROFILE).or({
        let homedrive = env::var(HOMEDRIVE);
        let homepath = env::var(HOMEPATH);
        homedrive.and_then(
            |x| homepath.map(
                |y| format!("{}{}", x, y)))
    });
    if home.is_err() {
        return Err(Error::new(ErrorKind::Other,
                format!("{}: home directory environment variables not found(%{}% or %{}% + %{}%)",
                    term::ewrite("failed")?, USERPROFILE, HOMEDRIVE, HOMEPATH)));
    }
    
    Ok(home.unwrap())
}
