use std::env;
use std::path::Path;

pub fn self_is_symlink() -> bool {
    let cwd = env::current_exe().unwrap();
    let path = Path::new(cwd.to_str().unwrap());

    path.read_link().is_ok()
}
