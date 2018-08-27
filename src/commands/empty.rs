use commands::Command;

pub struct EmptyCommand;

impl Command for EmptyCommand {
    fn execute(&self, _args: Vec<String>) -> String {
"command        Description
----------------------------------------------
init           Create an empty Git repository
add            Add file contents to the index
commit         Record changes to the repository".to_string()
    }
}