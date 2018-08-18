use std::env;

pub trait Command {
    fn execute(&self, mut args: env::Args) -> String;
}

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

pub struct InitCommand;

impl Command for InitCommand {
    fn execute(&self, mut args: env::Args) -> String {
        // implementation goes here
        let msg = "Initialized empty git repo".to_string();
        msg
    }
}

pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, mut args: env::Args) -> String {
        // implementation goes here
        let msg = "Index updated".to_string();
        msg
    }
}

pub struct CommitCommand;

impl Command for CommitCommand {
    fn execute(&self, mut args: env::Args) -> String {
        // implementation goes here
        let msg = "Committed...".to_string();
        msg // SHA-1
    }
}

pub struct MissingCommand;

impl Command for MissingCommand {
    fn execute(&self, mut args: env::Args) -> String {
        "mgit: unrecognized mgit command. See 'mgit' for help."
            .to_string()
    }
}
