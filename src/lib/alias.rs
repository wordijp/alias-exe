use std::{fs, io};

use crate::lib::path;

const LISTDIR: &str = "./list";

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
