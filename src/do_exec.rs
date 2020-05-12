use std::{env, process};

use crate::lib;

pub fn run(args: &Vec<String>) {
    let current_exe = env::current_exe().unwrap();
    let alias_name = current_exe.file_stem()
        .and_then(|x| x.to_str())
        .unwrap();

    let cfg_list_path = lib::path::cfg_list_path();
    if let Err(err) = cfg_list_path {
        eprintln!("{}", err);
        process::exit(1);
    };

    let value = lib::exec::read(&cfg_list_path.unwrap(), alias_name);
    if let Err(err) = value {
        eprintln!("{}", err);
        process::exit(1);
    }

    match lib::exec::run(&value.unwrap(), args) {
        Ok(status_code) => process::exit(status_code),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        },
    }
}
