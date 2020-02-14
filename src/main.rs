use std::env;

#[macro_use]
extern crate lazy_static;

mod path;
mod do_exec;
mod do_alias;

fn main() {
    let args: Vec<String> = env::args().collect();

    if path::self_is_symlink() {
        do_exec::run(&args);
    } else {
        do_alias::run(&args);
    }
}
