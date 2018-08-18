use std::env;
use commands::Command;

pub struct CommitCommand;

impl Command for CommitCommand {
    fn execute(&self, mut args: env::Args) -> String {

        //////////////////////////////
        // implementation goes here //
        //////////////////////////////

        "Committed...".to_string() // SHA-1
    }
}