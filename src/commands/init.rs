use commands::Command;

pub struct InitCommand;

impl Command for InitCommand {
    fn execute(&self, args: Vec<String>) -> String {

        //////////////////////////////
        // implementation goes here //
        //////////////////////////////

        "Initialized empty git repo".to_string()
    }
}