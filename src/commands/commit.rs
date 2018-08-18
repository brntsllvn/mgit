use std::env;
use commands::Command;

pub struct CommitCommand;

impl Command for CommitCommand {
    fn execute(&self, args: Vec<String>) -> String {

        //////////////////////////////
        // implementation goes here //
        //////////////////////////////

        "Committed...".to_string() // SHA-1
    }
}