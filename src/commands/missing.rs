use commands::Command;

pub struct MissingCommand;

impl Command for MissingCommand {
    fn execute(&self, args: Vec<String>) -> String {
        "mgit: unrecognized mgit command. See 'mgit' for help."
            .to_string()
    }
}