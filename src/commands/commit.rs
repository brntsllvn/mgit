use std::env;
use commands::Command;

pub struct CommitCommand;

impl Command for CommitCommand {
    fn execute(&self, mut args: env::Args) -> String {
        // implementation goes here
        let msg = "Committed...".to_string();
        msg // SHA-1
    }
}