use std::env;
use commands::Command;
use std::fs;

pub struct InitCommand;

impl Command for InitCommand {
    fn execute(&self, args: Vec<String>) -> String {

//        fs::create_dir("./.mgit");
//        fs::create_dir("./.mgit/objects");
//        fs::create_dir("./.mgit/refs");

        "Initialized empty git repo".to_string()
    }
}

// ./.git/objects
// ./.git/refs
// ./.git

use std::fs::File;
//use std::io::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_git_database() {
        let dummy_args: env::Args = Args::
        InitCommand::execute();
        let open_result = File::open("./.mgit");
    }
}