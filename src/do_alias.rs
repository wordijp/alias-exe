use clap::{
    App, SubCommand,
    crate_authors, crate_version
};

pub fn run() {
    // TODO: create alias
    // TODO: alias subcommand
    let app = App::new("alias")
        .author(crate_authors!())
        .version(crate_version!());

}
