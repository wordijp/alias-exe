use std::{env, fs};
use std::path::{self, Path};

use regex::Regex;

pub fn self_is_symlink() -> bool {
    let cwd = env::current_exe().unwrap();
    let path = Path::new(cwd.to_str().unwrap());

    path.read_link().is_ok()
}

lazy_static! {
    static ref RE_EXE: Regex = Regex::new(r"\.[eE][xX][eE]$").unwrap();
}

pub fn is_exe(path: &path::PathBuf) -> bool {
    let meta = fs::metadata(path);
    meta.is_ok() && meta.unwrap().is_file() && RE_EXE.is_match(path.to_str().unwrap())
}
