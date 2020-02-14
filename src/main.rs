mod path;
mod do_exec;
mod do_alias;

fn main() {
    if path::self_is_symlink() {
        do_exec::run();
    } else {
        do_alias::run();
    }
}
