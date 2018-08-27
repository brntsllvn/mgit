use commands::Command;

pub struct CommitCommand;

impl Command for CommitCommand {
    fn execute(&self, _args: Vec<String>) -> String {

        //////////////////////////////
        // implementation goes here //
        //////////////////////////////

        "Committed...".to_string() // SHA-1
    }
}