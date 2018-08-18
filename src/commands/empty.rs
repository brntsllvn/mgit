use std::env;
use commands::Command;

pub struct EmptyCommand;

impl Command for EmptyCommand {
    fn execute(&self, mut args: env::Args) -> String {
        let default = "command        Description
----------------------------------------------
init           Create an empty Git repository
add            Add file contents to the index
commit         Record changes to the repository";
        default.to_string()
    }
}