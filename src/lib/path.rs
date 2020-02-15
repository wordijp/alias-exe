use std::{env, fs, path};

use regex::Regex;

pub fn self_is_symlink() -> bool {
    env::current_exe().unwrap().read_link().is_ok()
}

lazy_static! {
    static ref RE_EXE: Regex = Regex::new(r"\.[eE][xX][eE]$").unwrap();
}

pub fn is_exe(path: &path::PathBuf) -> bool {
    RE_EXE.is_match(path.to_str().unwrap()) && fs::metadata(path).unwrap().is_file()
}
