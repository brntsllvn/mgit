use std::env;
pub mod empty;
pub mod init;
pub mod add;
pub mod commit;
pub mod missing;

pub trait Command {
    fn execute(&self, args: Vec<String>) -> String;
}

pub fn get_command(param: Option<String>) -> Box<Command> {
    match param {
        Some(txt) => match txt.as_ref() {
            "init" => Box::new(init::InitCommand) as Box<Command>,
            "add" => Box::new(add::AddCommand) as Box<Command>,
            "commit" => Box::new(commit::CommitCommand) as Box<Command>,
            _ => Box::new(missing::MissingCommand) as Box<Command>
        },
        None => Box::new(empty::EmptyCommand) as Box<Command>
    }
}
