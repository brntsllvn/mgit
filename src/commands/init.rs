use std::env;
use commands::Command;

pub struct InitCommand;

impl Command for InitCommand {
    fn execute(&self, mut args: env::Args) -> String {

        //////////////////////////////
        // implementation goes here //
        //////////////////////////////

        "Initialized empty git repo".to_string()
    }
}