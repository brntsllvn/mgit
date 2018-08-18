use std::env;
use commands::Command;

pub struct MissingCommand;

impl Command for MissingCommand {
    fn execute(&self, mut args: env::Args) -> String {
        "mgit: unrecognized mgit command. See 'mgit' for help."
            .to_string()
    }
}